module Uid exposing (..)

import Json.Decode as Decode exposing (..)
import Http


getUid : Http.Request String
getUid =
    Http.get "/web/uid" decodeUid


decodeUid : Decode.Decoder String
decodeUid =
    Decode.at [ "uid" ] Decode.string
