import { Client } from "client";

Client.prototype.first = function (value) {
  const next = value + 1;
  return next * 2;
};

Client.prototype.second = function (value) {
  const next = value + 1;
  return next * 2;
};

Client.prototype.different = function (value) {
  if (value) return 1;
  return 0;
};
