import { FastifyServer } from "../interface/server";

export const staticFiles = (server: FastifyServer) => {
  server.get("/", (request, reply) => {
    (reply as any).sendFile("/home.html");
  });
};
