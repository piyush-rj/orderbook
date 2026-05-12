import express from "express";
import router from "./src/routes";

const app = express();
app.use(express.json());

app.use("/api/v1", router);

app.listen(8080, () => {
    console.log("server running");
});
