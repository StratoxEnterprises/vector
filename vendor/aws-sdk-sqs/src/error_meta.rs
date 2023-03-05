// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>Two or more batch entries in the request have the same <code>Id</code>.</p>
    BatchEntryIdsNotDistinct(crate::error::BatchEntryIdsNotDistinct),
    /// <p>The length of all the messages put together is more than the limit.</p>
    BatchRequestTooLong(crate::error::BatchRequestTooLong),
    /// <p>The batch request doesn't contain any entries.</p>
    EmptyBatchRequest(crate::error::EmptyBatchRequest),
    /// <p>The specified attribute doesn't exist.</p>
    InvalidAttributeName(crate::error::InvalidAttributeName),
    /// <p>The <code>Id</code> of a batch entry in a batch request doesn't abide by the specification.</p>
    InvalidBatchEntryId(crate::error::InvalidBatchEntryId),
    /// <p>The specified receipt handle isn't valid for the current version.</p>
    InvalidIdFormat(crate::error::InvalidIdFormat),
    /// <p>The message contains characters outside the allowed set.</p>
    InvalidMessageContents(crate::error::InvalidMessageContents),
    /// <p>The specified message isn't in flight.</p>
    MessageNotInflight(crate::error::MessageNotInflight),
    /// <p>The specified action violates a limit. For example, <code>ReceiveMessage</code> returns this error if the maximum number of inflight messages is reached and <code>AddPermission</code> returns this error if the maximum number of permissions for the queue is reached.</p>
    OverLimit(crate::error::OverLimit),
    /// <p>Indicates that the specified queue previously received a <code>PurgeQueue</code> request within the last 60 seconds (the time it can take to delete the messages in the queue).</p>
    PurgeQueueInProgress(crate::error::PurgeQueueInProgress),
    /// <p>You must wait 60 seconds after deleting a queue before you can create another queue with the same name.</p>
    QueueDeletedRecently(crate::error::QueueDeletedRecently),
    /// <p>The specified queue doesn't exist.</p>
    QueueDoesNotExist(crate::error::QueueDoesNotExist),
    /// <p>A queue with this name already exists. Amazon SQS returns this error only if the request includes attributes whose values differ from those of the existing queue.</p>
    QueueNameExists(crate::error::QueueNameExists),
    /// <p>The specified receipt handle isn't valid.</p>
    ReceiptHandleIsInvalid(crate::error::ReceiptHandleIsInvalid),
    /// <p>The batch request contains more entries than permissible.</p>
    TooManyEntriesInBatchRequest(crate::error::TooManyEntriesInBatchRequest),
    /// <p>Error code 400. Unsupported operation.</p>
    UnsupportedOperation(crate::error::UnsupportedOperation),
    /// An unhandled error occurred.
    Unhandled(Box<dyn std::error::Error + Send + Sync + 'static>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BatchEntryIdsNotDistinct(inner) => inner.fmt(f),
            Error::BatchRequestTooLong(inner) => inner.fmt(f),
            Error::EmptyBatchRequest(inner) => inner.fmt(f),
            Error::InvalidAttributeName(inner) => inner.fmt(f),
            Error::InvalidBatchEntryId(inner) => inner.fmt(f),
            Error::InvalidIdFormat(inner) => inner.fmt(f),
            Error::InvalidMessageContents(inner) => inner.fmt(f),
            Error::MessageNotInflight(inner) => inner.fmt(f),
            Error::OverLimit(inner) => inner.fmt(f),
            Error::PurgeQueueInProgress(inner) => inner.fmt(f),
            Error::QueueDeletedRecently(inner) => inner.fmt(f),
            Error::QueueDoesNotExist(inner) => inner.fmt(f),
            Error::QueueNameExists(inner) => inner.fmt(f),
            Error::ReceiptHandleIsInvalid(inner) => inner.fmt(f),
            Error::TooManyEntriesInBatchRequest(inner) => inner.fmt(f),
            Error::UnsupportedOperation(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddPermissionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AddPermissionError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::AddPermissionErrorKind::OverLimit(inner) => Error::OverLimit(inner),
                crate::error::AddPermissionErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ChangeMessageVisibilityError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ChangeMessageVisibilityError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ChangeMessageVisibilityErrorKind::MessageNotInflight(inner) => {
                    Error::MessageNotInflight(inner)
                }
                crate::error::ChangeMessageVisibilityErrorKind::ReceiptHandleIsInvalid(inner) => {
                    Error::ReceiptHandleIsInvalid(inner)
                }
                crate::error::ChangeMessageVisibilityErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ChangeMessageVisibilityBatchError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ChangeMessageVisibilityBatchError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, ..} => match err.kind {
                crate::error::ChangeMessageVisibilityBatchErrorKind::BatchEntryIdsNotDistinct(inner) => Error::BatchEntryIdsNotDistinct(inner),
                crate::error::ChangeMessageVisibilityBatchErrorKind::EmptyBatchRequest(inner) => Error::EmptyBatchRequest(inner),
                crate::error::ChangeMessageVisibilityBatchErrorKind::InvalidBatchEntryId(inner) => Error::InvalidBatchEntryId(inner),
                crate::error::ChangeMessageVisibilityBatchErrorKind::TooManyEntriesInBatchRequest(inner) => Error::TooManyEntriesInBatchRequest(inner),
                crate::error::ChangeMessageVisibilityBatchErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            }
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreateQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreateQueueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::CreateQueueErrorKind::QueueDeletedRecently(inner) => {
                    Error::QueueDeletedRecently(inner)
                }
                crate::error::CreateQueueErrorKind::QueueNameExists(inner) => {
                    Error::QueueNameExists(inner)
                }
                crate::error::CreateQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteMessageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteMessageError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteMessageErrorKind::InvalidIdFormat(inner) => {
                    Error::InvalidIdFormat(inner)
                }
                crate::error::DeleteMessageErrorKind::ReceiptHandleIsInvalid(inner) => {
                    Error::ReceiptHandleIsInvalid(inner)
                }
                crate::error::DeleteMessageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteMessageBatchError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeleteMessageBatchError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteMessageBatchErrorKind::BatchEntryIdsNotDistinct(inner) => {
                    Error::BatchEntryIdsNotDistinct(inner)
                }
                crate::error::DeleteMessageBatchErrorKind::EmptyBatchRequest(inner) => {
                    Error::EmptyBatchRequest(inner)
                }
                crate::error::DeleteMessageBatchErrorKind::InvalidBatchEntryId(inner) => {
                    Error::InvalidBatchEntryId(inner)
                }
                crate::error::DeleteMessageBatchErrorKind::TooManyEntriesInBatchRequest(inner) => {
                    Error::TooManyEntriesInBatchRequest(inner)
                }
                crate::error::DeleteMessageBatchErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeleteQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeleteQueueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::DeleteQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetQueueAttributesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetQueueAttributesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetQueueAttributesErrorKind::InvalidAttributeName(inner) => {
                    Error::InvalidAttributeName(inner)
                }
                crate::error::GetQueueAttributesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetQueueUrlError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::GetQueueUrlError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::GetQueueUrlErrorKind::QueueDoesNotExist(inner) => {
                    Error::QueueDoesNotExist(inner)
                }
                crate::error::GetQueueUrlErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListDeadLetterSourceQueuesError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ListDeadLetterSourceQueuesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListDeadLetterSourceQueuesErrorKind::QueueDoesNotExist(inner) => {
                    Error::QueueDoesNotExist(inner)
                }
                crate::error::ListDeadLetterSourceQueuesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListQueuesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListQueuesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListQueuesErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListQueueTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListQueueTagsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ListQueueTagsErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PurgeQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PurgeQueueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::PurgeQueueErrorKind::PurgeQueueInProgress(inner) => {
                    Error::PurgeQueueInProgress(inner)
                }
                crate::error::PurgeQueueErrorKind::QueueDoesNotExist(inner) => {
                    Error::QueueDoesNotExist(inner)
                }
                crate::error::PurgeQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ReceiveMessageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ReceiveMessageError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::ReceiveMessageErrorKind::OverLimit(inner) => Error::OverLimit(inner),
                crate::error::ReceiveMessageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RemovePermissionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::RemovePermissionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::RemovePermissionErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendMessageError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SendMessageError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendMessageErrorKind::InvalidMessageContents(inner) => {
                    Error::InvalidMessageContents(inner)
                }
                crate::error::SendMessageErrorKind::UnsupportedOperation(inner) => {
                    Error::UnsupportedOperation(inner)
                }
                crate::error::SendMessageErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SendMessageBatchError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SendMessageBatchError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SendMessageBatchErrorKind::BatchEntryIdsNotDistinct(inner) => {
                    Error::BatchEntryIdsNotDistinct(inner)
                }
                crate::error::SendMessageBatchErrorKind::BatchRequestTooLong(inner) => {
                    Error::BatchRequestTooLong(inner)
                }
                crate::error::SendMessageBatchErrorKind::EmptyBatchRequest(inner) => {
                    Error::EmptyBatchRequest(inner)
                }
                crate::error::SendMessageBatchErrorKind::InvalidBatchEntryId(inner) => {
                    Error::InvalidBatchEntryId(inner)
                }
                crate::error::SendMessageBatchErrorKind::TooManyEntriesInBatchRequest(inner) => {
                    Error::TooManyEntriesInBatchRequest(inner)
                }
                crate::error::SendMessageBatchErrorKind::UnsupportedOperation(inner) => {
                    Error::UnsupportedOperation(inner)
                }
                crate::error::SendMessageBatchErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SetQueueAttributesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::SetQueueAttributesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::SetQueueAttributesErrorKind::InvalidAttributeName(inner) => {
                    Error::InvalidAttributeName(inner)
                }
                crate::error::SetQueueAttributesErrorKind::Unhandled(inner) => {
                    Error::Unhandled(inner)
                }
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::TagQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::TagQueueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::TagQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::UntagQueueError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::UntagQueueError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError { err, .. } => match err.kind {
                crate::error::UntagQueueErrorKind::Unhandled(inner) => Error::Unhandled(inner),
            },
            _ => Error::Unhandled(err.into()),
        }
    }
}
impl std::error::Error for Error {}