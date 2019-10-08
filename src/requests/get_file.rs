use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::File,
};
use std::borrow::Cow;

/// Use this method to get basic info about a file and prepare it for
/// downloading. For the moment, bots can download files of up to 20MB in size.
/// On success, a File object is returned.
/// The file can then be downloaded via the link https://api.telegram.org/file/bot<token>/<file_path>,
/// where <file_path> is taken from the response.
/// It is guaranteed that the link will be valid for at least 1 hour.
/// When the link expires, a new one can be requested by calling getFile again.
#[derive(Debug, Clone, Serialize)]
pub struct GetFile<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// File identifier to get info about
    pub file_id: Cow<'a, str>,
}

#[async_trait]
impl Request for GetFile<'_> {
    type ReturnValue = File;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl GetFile<'_> {
    pub async fn send(self) -> ResponseResult<File> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "getFile",
            &self,
        )
        .await
    }
}

impl<'a> GetFile<'a> {
    pub(crate) fn new(ctx: RequestContext<'a>, file_id: Cow<'a, str>) -> Self {
        Self { ctx, file_id }
    }

    pub fn file_id<C>(mut self, file_id: C) -> Self
    where
        C: Into<Cow<'a, str>>,
    {
        self.file_id = file_id.into();
        self
    }
}
