module NotebookHistory exposing (..)

import Json.Decode as Decode exposing (..)
import Http


type alias Model =
    { id : Int
    , notebook_id : Int
    }


createNotebookHistory: Int -> Http.Request Model
createNotebookHistory notebook_id =
    Http.post
        ("/web/notebooks/" ++ toString notebook_id ++ "/notebook_histories")
        Http.emptyBody
        decodeNotebookHistory


decodeNotebookHistory : Decode.Decoder Model
decodeNotebookHistory  =
    map2 Model (field "id" int) (field "notebook_id" int)
