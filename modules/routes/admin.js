// Allow require
import { createRequire } from "module";
const require = createRequire(import.meta.url);

var express = require('express');
var router = express.Router();


router.get('/admin', (req, res) => {

  //Database(req.body.username, req.body.password);

  res.json({
    "status": "Success",
  });
});

export default router;