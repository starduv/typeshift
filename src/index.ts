
export type OperationMethod = 'GET' | 'PUT' | 'POST' | 'DELETE' | 'OPTIONS' | 'HEAD' | 'PATCH' | 'TRACE';

export interface PathItemOptions {
    method: OperationMethod;
    path: string;
    tags?: string[];
}

export function Path<Func>(fn: Func, options: PathItemOptions | null = null) {
    return fn;
}

export interface ResponseOptions {
    description?: string;
    example?: string;
    namespace?: string;
    statusCode: number;
}

export function Response<ResponseType>(response: ResponseType, options: ResponseOptions) {
    return response;
}

type NS = string | undefined;
type F = StringFormat | NumberFormat;
type StringFormat = "date-time" |
    "time" |
    "date" |
    "duration" |
    "email" |
    "idn-email" |
    "hostname" |
    "idn-hostname" |
    "ipv4" |
    "ipv6" |
    "uuid" |
    "uri" |
    "uri-reference" |
    "uri-template" |
    "json-pointer" |
    "relative-json-pointer" |
    "regex" |
    "iri" |
    "iri-reference" |
    undefined;

type NumberFormat = "int32" | "int64" | "float" | "double"

export type BodyParam<Param, Required extends boolean, Namespace extends NS = undefined> = Param;
export type Header<Param, Required extends boolean, Namespace extends NS = undefined, Format extends F = undefined> = Param;
export type QueryParam<Param, Required extends boolean, Namespace extends NS = undefined, Format extends F = undefined> = Param;
export type RouteParam<Param, Required extends true, Namespace extends NS = undefined, Format extends F = undefined> = Param;