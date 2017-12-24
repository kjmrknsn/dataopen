module Notebook exposing (..)

import Json.Decode as Decode exposing (..)
import Http


createNotebook: Http.Request String
createNotebook =
    Http.post "/web/notebooks" Http.emptyBody decodeNotebook


decodeNotebook : Decode.Decoder String
decodeNotebook  =
    Decode.at [ "id" ] Decode.string
