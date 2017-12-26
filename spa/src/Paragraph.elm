module Paragraph exposing (..)

import Json.Decode as Decode
import Http


type alias Model =
    { id: Int
    , notebookId: Int
    , notebookHistoryId: Int
    , code: String
    , result: String
    }


getParagraphs : Int -> Int -> Http.Request (List Model)
getParagraphs notebookId notebookHistoryId =
    Http.get
        ( "/web/notebooks/"
        ++ toString notebookId
        ++ "/notebook_histories/"
        ++ toString notebookHistoryId
        ++ "/paragraphs")
        decodeParagraphs


decodeParagraphs : Decode.Decoder (List Model)
decodeParagraphs =
    Decode.list decode


decode : Decode.Decoder Model
decode =
    Decode.map5 Model
        (Decode.field "id" Decode.int)
        (Decode.field "notebookId" Decode.int)
        (Decode.field "notebookHistoryId" Decode.int)
        (Decode.field "code" Decode.string)
        (Decode.field "result" Decode.string)
