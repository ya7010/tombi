import {
  RequestType,
  type TextDocumentIdentifier,
} from "vscode-languageclient";

export type GetTomlVersionParams = TextDocumentIdentifier;
export const getTomlVersion = new RequestType<
  GetTomlVersionParams,
  string,
  void
>("tombi/getTomlVersion");

export type UpdateSchemaParams = TextDocumentIdentifier;
export const updateSchema = new RequestType<UpdateSchemaParams, boolean, void>(
  "tombi/updateSchema",
);
