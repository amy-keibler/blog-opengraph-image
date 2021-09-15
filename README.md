# Blog Image

A companion tool for generating [OpenGraph](https://ogp.me/) compatible images for [Zola](https://www.getzola.org/) static websites.

Given a filepath to a markdown document that is formatted for use in `zola`, the tool generates a `.png` file with the same name in the same folder.

The `title` and `tags` fields from the header section (the section surrounded with `+++`) are used, but the tool will be customized in the future to include the ability to overwrite the `title` and pull in additional fields.

The `tags` are used to see if any custom backgrounds should be used, with a fallback of a default random bubbles background. Right now, `rust` fills the background with Ferris images.

Currently, this is heavily customized to [my blog](https://blog.amy-k.net/)'s use-case.
