use std::rc::Rc;

use async_trait::async_trait;
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue};
use hyperx::header::{Link, RelationType};
use serde_json::Value;

use crate::{ZoteroApiAsyncExecutor, ZoteroApiError, ZoteroApiExecutor};

impl ZoteroApiExecutor for http::Request<Bytes> {
    fn fetch_headers<I>(self, query: Option<I>) -> Result<HeaderMap, crate::ZoteroApiError>
    where
        I: IntoIterator<Item = HeaderName>,
    {
        let client = reqwest::blocking::Client::new();

        let res = client
            .execute(self.try_into().unwrap())
            .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

        if res.status().as_u16() == 403 {
            return Err(ZoteroApiError::AuthenticationError(
                res.text().unwrap_or("".to_string()),
            ));
        }

        if let Some(query) = query {
            let headers = res.headers();
            let ret = query.into_iter().fold(HeaderMap::new(), |mut curr, nxt| {
                if let Some(val) = headers.get(nxt.clone()) {
                    curr.insert(nxt, val.to_owned());
                }
                curr
            });
            Ok(ret)
        } else {
            Ok(res.headers().clone())
        }
    }

    fn fetch_header(
        self,
        header_name: HeaderName,
    ) -> Result<Option<HeaderValue>, crate::ZoteroApiError> {
        <Self as ZoteroApiExecutor>::fetch_headers(self, Some(vec![header_name.clone()]))
            .and_then(|m| Ok(m.get(header_name).map(ToOwned::to_owned)))
    }

    fn execute<'a, T: serde::Deserialize<'a>, Z: crate::ZoteroApi<'a>>(
        self,
        zotero_api: &Z,
    ) -> Result<T, crate::ZoteroApiError> {
        let client = reqwest::blocking::Client::new();

        let res = client
            .execute(self.try_into().unwrap())
            .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

        if res.status().as_u16() == 403 {
            return Err(ZoteroApiError::AuthenticationError(
                res.text().unwrap_or("".to_string()),
            ));
        }

        let mut next_page = get_next_page(res.headers().clone());

        let mut response = res
            .json::<Value>()
            .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;

        match next_page {
            None => {
                let response = Rc::new(response);
                let err_response = Rc::clone(&response);

                T::deserialize(Rc::unwrap_or_clone(response)).map_err(|err| {
                    ZoteroApiError::DeserializeResponseError {
                        msg: err.to_string(),
                        value: Rc::unwrap_or_clone(err_response),
                    }
                })
            }
            Some(_) => {
                let mut responses: Vec<Value> = vec![];

                let response =
                    response
                        .as_array_mut()
                        .ok_or(ZoteroApiError::ParseResponseError(
                            "Expected response to be an array.".to_string(),
                        ))?;
                responses.append(response);

                // follow pagination if any
                while let Some(np) = next_page {
                    let res = client
                        .execute(zotero_api.request_uri("GET", np).try_into().unwrap())
                        .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

                    next_page = get_next_page(res.headers().clone());

                    let mut response: Vec<Value> = res
                        .json()
                        .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;

                    responses.append(&mut response);
                }

                let responses = Rc::new(Value::Array(responses));
                let err_responses = Rc::clone(&responses);

                T::deserialize(Rc::unwrap_or_clone(responses)).map_err(|err| {
                    ZoteroApiError::DeserializeResponseError {
                        msg: format!("{err:?}"),
                        value: Rc::unwrap_or_clone(err_responses),
                    }
                })
            }
        }
    }
}

#[async_trait]
impl ZoteroApiAsyncExecutor for http::Request<Bytes> {
    async fn fetch_headers<I>(self, query: Option<I>) -> Result<HeaderMap, crate::ZoteroApiError>
    where
        I: IntoIterator<Item = HeaderName> + Send,
    {
        let client = reqwest::Client::new();

        let res = client
            .execute(self.try_into().unwrap())
            .await
            .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

        if res.status().as_u16() == 403 {
            return Err(ZoteroApiError::AuthenticationError(
                res.text().await.unwrap_or("".to_string()),
            ));
        }

        if let Some(query) = query {
            let headers = res.headers();
            let ret = query.into_iter().fold(HeaderMap::new(), |mut curr, nxt| {
                if let Some(val) = headers.get(nxt.clone()) {
                    curr.insert(nxt, val.to_owned());
                }
                curr
            });
            Ok(ret)
        } else {
            Ok(res.headers().clone())
        }
    }

    async fn fetch_header(
        self,
        header_name: HeaderName,
    ) -> Result<Option<HeaderValue>, crate::ZoteroApiError> {
        <Self as ZoteroApiAsyncExecutor>::fetch_headers(self, Some(vec![header_name.clone()]))
            .await
            .and_then(|m| Ok(m.get(header_name).map(ToOwned::to_owned)))
    }

    async fn execute<'a, T: serde::Deserialize<'a>, Z: crate::ZoteroApi<'a> + std::marker::Sync>(
        self,
        zotero_api: &Z,
    ) -> Result<T, crate::ZoteroApiError> {
        let client = reqwest::Client::new();

        let res = client
            .execute(self.try_into().unwrap())
            .await
            .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

        if res.status().as_u16() == 403 {
            return Err(ZoteroApiError::AuthenticationError(
                res.text().await.unwrap_or("".to_string()),
            ));
        }

        let mut next_page = get_next_page(res.headers().clone());

        let mut response = res
            .json::<Value>()
            .await
            .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;

        match next_page {
            None => {
                let response = Rc::new(response);
                let err_response = Rc::clone(&response);

                T::deserialize(Rc::unwrap_or_clone(response)).map_err(|err| {
                    ZoteroApiError::DeserializeResponseError {
                        msg: err.to_string(),
                        value: Rc::unwrap_or_clone(err_response),
                    }
                })
            }

            Some(_) => {
                let mut responses: Vec<Value> = vec![];

                let response =
                    response
                        .as_array_mut()
                        .ok_or(ZoteroApiError::ParseResponseError(
                            "Expected response to be an array.".to_string(),
                        ))?;
                responses.append(response);

                // follow pagination if any
                while let Some(np) = next_page {
                    let res = client
                        .execute(zotero_api.request_uri("GET", np).try_into().unwrap())
                        .await
                        .map_err(|err| ZoteroApiError::RequestError(err.to_string()))?;

                    next_page = get_next_page(res.headers().clone());

                    let mut response: Vec<Value> = res
                        .json()
                        .await
                        .map_err(|err| ZoteroApiError::ParseResponseError(err.to_string()))?;

                    responses.append(&mut response);
                }

                let responses = Rc::new(Value::Array(responses));
                let err_responses = Rc::clone(&responses);

                T::deserialize(Rc::unwrap_or_clone(responses)).map_err(|err| {
                    ZoteroApiError::DeserializeResponseError {
                        msg: format!("{err:?}"),
                        value: Rc::unwrap_or_clone(err_responses),
                    }
                })
            }
        }
    }
}

fn get_next_page(headers: reqwest::header::HeaderMap) -> Option<String> {
    headers.get(reqwest::header::LINK).and_then(|link_header| {
        link_header
            .to_str()
            .unwrap_or("")
            .parse()
            .ok()
            .and_then(|link: Link| {
                link.values()
                    .iter()
                    .find(|link_value| {
                        link_value
                            .rel()
                            .is_some_and(|rel| rel.contains(&RelationType::Next))
                    })
                    .map(|link_value| link_value.link().to_string())
            })
    })
}
