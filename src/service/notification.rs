use std::thread;

use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::model::{notification::Notification, subscriber};
use crate::repository::subscriber::SubscriberRepository;
use bambangshop::{compose_error_response, Result};
use rocket::http::Status;

pub struct NotificationService;

impl NotificationService {}