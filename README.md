HTML2MD
============

Library to convert simple html documents into markdown flavor.
Implements markdown as written on its [inception page](https://daringfireball.net/projects/markdown).

Features
------------

Currently supported:

+ Lists (and inner lists)
+ Headers
+ Quotes (and inner quotes)
+ Paragraphs
+ Horizontal rulers
+ Images and links
+ Tables
+ Formatting (bold, italic, strikethrough, underline)
+ Code

Limitations
-------------

- no markdown flavors support (-/+ unordered list styles, ##/== headers etc.)
- doesn't yet detect code style

Used libraries
-------------
[html5ever](https://github.com/servo/html5ever) - Servo egine HTML parsing library, used to convert html input to DOM

[regex](https://github.com/rust-lang/regex) - PCRE support in Rust, used to correct whitespaces

Contributions
------------
You may create merge request or bug/enhancement issue right here on GitLab, or send formatted patch via e-mail. 
For details see CONTRIBUTING.md file in this repo. 

License
-------------

    Copyright (C) 2018-2019  Oleg `Kanedias` Chernovskiy

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
