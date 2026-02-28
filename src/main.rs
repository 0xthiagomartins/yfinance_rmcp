use rmcp::{
    handler::server::tool::ToolRouter,
    model::{CallToolResult, Content, Parameters, ServerCapabilities, ServerHandler, ServerInfo},
    tool, tool_handler, tool_router,
    transport::io::stdio,
    ErrorData as McpError,
    ServiceExt,
};
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Clone)]
struct YahooFinanceHandler {
    tool_router: ToolRouter<Self>,
}

#[derive(Deserialize, JsonSchema)]
struct HistoricalPricesParams {
    /// Ticker symbol (e.g. AAPL)
    ticker: String,
    /// Period: 1d, 5d, 1mo, 3mo, 6mo, 1y, 2y, 5y, 10y, ytd, max. Default 1mo
    #[serde(default = "default_period")]
    period: String,
    /// Interval: 1m, 2m, 5m, 15m, 30m, 60m, 90m, 1h, 1d, 5d, 1wk, 1mo, 3mo. Default 1d
    #[serde(default = "default_interval")]
    interval: String,
}

fn default_period() -> String {
    "1mo".into()
}
fn default_interval() -> String {
    "1d".into()
}

#[derive(Deserialize, JsonSchema)]
struct QuoteParams {
    /// Ticker symbol (e.g. AAPL)
    ticker: String,
}

#[derive(Deserialize, JsonSchema)]
struct SearchParams {
    /// Search query (e.g. company name or symbol)
    query: String,
}

#[tool_router]
impl YahooFinanceHandler {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        name = "get_historical_stock_prices",
        description = "Get historical OHLCV data for a stock. Returns Date, Open, High, Low, Close, Volume."
    )]
    async fn get_historical_stock_prices(
        &self,
        params: Parameters<HistoricalPricesParams>,
    ) -> Result<CallToolResult, McpError> {
        let p = params.0;
        let provider = match yahoo_finance_api::YahooConnector::new() {
            Ok(conn) => conn,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Yahoo connector error: {}",
                    e
                )]));
            }
        };
        let response = match provider
            .get_quote_range(&p.ticker, &p.interval, &p.period)
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Failed to get quotes for {}: {}",
                    p.ticker, e
                )]));
            }
        };
        let quotes = match response.quotes() {
            Some(q) => q,
            None => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "No quote data for {}",
                    p.ticker
                )]));
            }
        };
        let records: Vec<serde_json::Value> = quotes
            .iter()
            .map(|q| {
                serde_json::json!({
                    "date": q.timestamp,
                    "open": q.open,
                    "high": q.high,
                    "low": q.low,
                    "close": q.close,
                    "volume": q.volume
                })
            })
            .collect();
        let json = serde_json::to_string(&records).unwrap_or_else(|_| "[]".into());
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }

    #[tool(
        name = "get_stock_quote",
        description = "Get latest quote for a ticker (open, high, low, close, volume)."
    )]
    async fn get_stock_quote(&self, params: Parameters<QuoteParams>) -> Result<CallToolResult, McpError> {
        let ticker = params.0.ticker;
        let provider = match yahoo_finance_api::YahooConnector::new() {
            Ok(conn) => conn,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Yahoo connector error: {}",
                    e
                )]));
            }
        };
        let response = match provider.get_latest_quotes(&ticker, "1d").await {
            Ok(r) => r,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Failed to get quote for {}: {}",
                    ticker, e
                )]));
            }
        };
        let quote = match response.last_quote() {
            Some(q) => q,
            None => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "No quote for {}",
                    ticker
                )]));
            }
        };
        let json = serde_json::json!({
            "ticker": ticker,
            "timestamp": quote.timestamp,
            "open": quote.open,
            "high": quote.high,
            "low": quote.low,
            "close": quote.close,
            "volume": quote.volume
        });
        Ok(CallToolResult::success(vec![Content::text(
            serde_json::to_string(&json).unwrap(),
        )]))
    }

    #[tool(
        name = "search_ticker",
        description = "Search for ticker symbols by company name or keyword."
    )]
    async fn search_ticker(&self, params: Parameters<SearchParams>) -> Result<CallToolResult, McpError> {
        let query = params.0.query;
        let provider = match yahoo_finance_api::YahooConnector::new() {
            Ok(conn) => conn,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Yahoo connector error: {}",
                    e
                )]));
            }
        };
        let response = match provider.search_ticker(&query).await {
            Ok(r) => r,
            Err(e) => {
                return Ok(CallToolResult::error(vec![Content::text(format!(
                    "Search failed: {}",
                    e
                )]));
            }
        };
        let list: Vec<serde_json::Value> = response
            .quotes
            .iter()
            .map(|q| {
                serde_json::json!({
                    "symbol": q.symbol,
                    "short_name": q.short_name,
                    "long_name": q.long_name,
                    "exchange": q.exchange,
                    "quote_type": q.quote_type
                })
            })
            .collect();
        let json = serde_json::to_string(&list).unwrap_or_else(|_| "[]".into());
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[tool_handler]
impl ServerHandler for YahooFinanceHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            name: "yfinance_rmcp".into(),
            version: Some("0.1.0".into()),
            instructions: Some(
                "Yahoo Finance data collector. Tools: get_historical_stock_prices, get_stock_quote, search_ticker.".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = YahooFinanceHandler::new()
        .serve(stdio())
        .await
        .inspect_err(|e| eprintln!("MCP error: {}", e))?;
    service.waiting().await?;
    Ok(())
}
