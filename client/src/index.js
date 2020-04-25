"use strict";

import '../css/main.scss';
import '@fortawesome/fontawesome-free/css/all.css';

import("../pkg").then(module => {
  module.run_app();
});
