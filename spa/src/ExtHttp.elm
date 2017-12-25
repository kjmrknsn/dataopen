module ExtHttp exposing (..)

import Json.Decode as Decode
import Http exposing (Body, Request, expectJson, request)


patch : String -> Body -> Decode.Decoder a -> Request a
patch url body decoder =
  request
    { method = "PATCH"
    , headers = []
    , url = url
    , body = body
    , expect = expectJson decoder
    , timeout = Nothing
    , withCredentials = False
    }


put : String -> Body -> Decode.Decoder a -> Request a
put url body decoder =
  request
    { method = "PATCH"
    , headers = []
    , url = url
    , body = body
    , expect = expectJson decoder
    , timeout = Nothing
    , withCredentials = False
    }
