module NotebookHistory exposing (..)

import Json.Decode as Decode exposing (..)
import Http


type alias Model =
    { id : Int
    , notebookId : Int
    , title : String
    }


new : Model
new =
    { id = 0
    , notebookId = 0
    , title = ""
    }


updateTitle: Model -> String -> Model
updateTitle model title =
    { model | title = title }


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
    map3 Model (field "id" int) (field "notebookId" int) (field "title" string)
