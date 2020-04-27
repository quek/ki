"use strict";

import '../css/main.scss';
import '@fortawesome/fontawesome-free/css/all.css';
import 'prismjs';
import 'prismjs/themes/prism-okaidia.css';
import 'prismjs/components/prism-markup';
import 'prismjs/components/prism-rust';

import("../pkg").then(module => {
  module.run_app();
});
