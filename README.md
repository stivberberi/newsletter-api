# Rust Newsletter API

This newsletter API is based on the book by Lucas Palmieri; [Zero to Production in Rust](https://www.zero2prod.com/index.html?country_code=CA)

## User Stories
The goal of this project will be to fulfill the following "user stories":

* As a blog visitor, I want to subscribe to the newsletter, so that I can receive email updates when new content is published to the blog.
* As a blog author, I want to send an email to all my subscribers, so that  I can notify them when new content is published.
* As a subscriber, I want to be able to unsubscribe from the newsletter, so that I can stop receiving email updates from the blog. 

## Features to add
As stated in the book, the following features will not be added, which may be fun to try implementing once the barebones project is complete:

* Manage multiple newsletters
* Segment subscribers into multiple audiences
* Track opening and click rates

## Web Framework
Following the book, [actix-web](https://actix.rs/) will be used for this project. [Axum](https://docs.rs/axum/latest/axum/) is another good, more modern choice.
