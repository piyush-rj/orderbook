import express from "express";
import router from "./src/routes/routes";
import services from "./src/services/services.init";
import { parse_env } from "./src/config/config.env";

parse_env();

const app = express();
app.use(express.json());

app.use("/api/v1", router);

app.listen(8080, () => {
    services.log_api_boot();
    console.log("server running");
});
