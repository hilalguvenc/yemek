import { FastifyServer } from "../interface/server";
import axios from "axios";

export const projectsApi = (server: FastifyServer) => {
  server.get<{ Params: { id: string } }>(
    "/api/projects/:id",
    async (request, reply) => {
      const response = await axios.get(
        `https://web-assignment-api.zeplin.xyz/projects/${request.params.id}`
      );
      reply.status(response.status).send(response.data);
    }
  );

  server.get<{ Params: { projectid: string; screenid: string } }>(
    "/api/projects/:projectid/screens/:screenid",
    async (request, reply) => {
      const response = await axios.get(
        `https://web-assignment-api.zeplin.xyz/projects/${request.params.projectid}/screens/${request.params.screenid}/dots`
      );
      reply.status(response.status).send(response.data);
    }
  );
};
