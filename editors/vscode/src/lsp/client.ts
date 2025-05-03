import {
  RequestType,
  type TextDocumentIdentifier,
} from "vscode-languageclient";

export type GetTomlVersionParams = TextDocumentIdentifier;
export const getTomlVersion = new RequestType<
  GetTomlVersionParams,
  { tomlVersion: string; source: "config" | "schema" | "default" },
  void
>("tombi/getTomlVersion");

export type UpdateConfigParams = TextDocumentIdentifier;
export const updateConfig = new RequestType<UpdateConfigParams, boolean, void>(
  "tombi/updateConfig",
);

export type UpdateSchemaParams = TextDocumentIdentifier;
export const updateSchema = new RequestType<UpdateSchemaParams, boolean, void>(
  "tombi/updateSchema",
);

export type AssociateSchemaParams = {
  url: string;
  fileMatch: string[];
};
export const associateSchema = new RequestType<
  AssociateSchemaParams,
  void,
  void
>("tombi/associateSchema");
