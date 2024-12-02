const session_url_path: &str = "/session";

struct Error {
	error: String;
	message: String;
	stacktrace: String;
}

struct ProxyConfiguration {
	proxy_type: String;
	proxy_auto_config_url: String;
	ftp_proxy: String;
	http_proxy: String;
	no_proxy: Vec<String>;
	ssl_proxy: String;
	socks_proxy: String;
	socks_version: usize;
}

struct SessionCapabilities {
	browser_name: String;
	browser_version: String;
	platform_name: String;
	accepts_insecure_certs: bool;
	page_load_strategy: String;
	proxy: ProxyConfiguration;
	set_window_rect: bool;
	strict_file_interactability: bool;
	unhandled_prompt_behavior: String;
	user_agent: String;
}

struct NewSessionJSONResposne {
	session_id: String,
	capabilities: SessionCapabilities;
}

// https://www.w3.org/TR/webdriver2/#dfn-new-sessions
pub fn new_session_request() {
	// add POST
	//
	//

	// print response at first
}

// https://www.w3.org/TR/webdriver2/#delete-session
pub fn delete_session_request(session_id: String) {
	// add DELETE
	// /session/<session_id>
	//
	// success if return value is null

	// print response at first
}
