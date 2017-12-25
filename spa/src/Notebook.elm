module Notebook exposing (..)

import Json.Decode as Decode exposing (..)
import Http


createNotebook: Http.Request Int
createNotebook =
    Http.post "/web/notebooks" Http.emptyBody decodeNotebook


decodeNotebook : Decode.Decoder Int
decodeNotebook  =
    Decode.at [ "id" ] Decode.int
