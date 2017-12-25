module NotebookHistory exposing (..)

import Json.Decode as Decode exposing (..)
import Http


type alias Model =
    { id : Int
    , notebookId : Int
    }


new : Int -> Int -> Model
new id notebookId =
    { id = id
    , notebookId = notebookId
    }


getNotebookHistory: Int -> Int -> Http.Request Model
getNotebookHistory notebookId id =
    Http.get
        ("/web/notebooks/" ++ toString notebookId ++ "/notebook_histories/" ++ toString id)
        decodeNotebookHistory


createNotebookHistory: Int -> Http.Request Model
createNotebookHistory notebookId =
    Http.post
        ("/web/notebooks/" ++ toString notebookId ++ "/notebook_histories")
        Http.emptyBody
        decodeNotebookHistory


decodeNotebookHistory : Decode.Decoder Model
decodeNotebookHistory  =
    map2 Model (field "id" int) (field "notebookId" int)