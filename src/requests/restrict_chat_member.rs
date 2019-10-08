use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::{ChatPermissions, True},
};

/// Use this method to restrict a user in a supergroup. The bot must be an
/// administrator in the supergroup for this to work and must have the
/// appropriate admin rights. Pass True for all permissions to lift restrictions
/// from a user. Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct RestrictChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///Unique identifier for the target chat or username of the target
    /// supergroup (in the format @supergroupusername)
    pub chat_id: ChatId<'a>,
    ///Unique identifier of the target user
    pub user_id: i32,
    ///New user permissions
    pub permissions: ChatPermissions,
    ///Date when restrictions will be lifted for the user, unix time. If user
    /// is restricted for more than 366 days or less than 30 seconds from the
    /// current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

#[async_trait]
impl<'a> Request for RestrictChatMember<'a> {
    type ReturnValue = True;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl RestrictChatMember<'_> {
    async fn send(self) -> ResponseResult<True> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "restrictChatMember",
            &self,
        )
        .await
    }
}

impl<'a> RestrictChatMember<'a> {
    pub(crate) fn new<C, U>(
        ctx: RequestContext<'a>,
        chat_id: C,
        user_id: U,
        permissions: ChatPermissions,
    ) -> Self
    where
        C: Into<ChatId<'a>>,
        U: Into<i32>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            user_id,
            permissions,
            until_date: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId<'a>>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn user_id<T>(mut self, user_id: T) -> Self
    where
        T: Into<i32>,
    {
        self.user_id = user_id.into();
        self
    }

    pub fn permissions<T>(mut self, permissions: T) -> Self
    where
        T: Into<ChatPermissions>,
    {
        self.permissions = permissions.into();
        self
    }

    pub fn until_date<T>(mut self, until_date: T) -> Self
    where
        T: Into<u64>,
    {
        self.until_date = Some(until_date.into());
        self
    }
}
