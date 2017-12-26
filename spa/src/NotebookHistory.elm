module NotebookHistory exposing (..)

import ExtHttp
import Json.Decode as Decode
import Json.Encode as Encode
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


updateTitleOnLocal: Model -> String -> Model
updateTitleOnLocal model title =
    { model | title = title }


updateTitle : Model -> Http.Request Decode.Value
updateTitle notebookHistory =
    let
        body =
            { title = notebookHistory.title }
                |> notebookHistoryTitleEncoder
                |> Http.jsonBody
    in
        ExtHttp.patch
            ( "/web/notebooks/"
            ++ toString notebookHistory.notebookId
            ++ "/notebook_histories/"
            ++ toString notebookHistory.id
            ++ "/title"
            )
            body
            Decode.value


complete : Model -> Http.Request Decode.Value
complete notebookHistory =
    ExtHttp.patch
        ( "/web/notebooks/"
        ++ toString notebookHistory.notebookId
        ++ "/notebook_histories/"
        ++ toString notebookHistory.id
        ++ "/complete"
        )
        Http.emptyBody
        Decode.value


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
    Decode.map3 Model
        (Decode.field "id" Decode.int)
        (Decode.field "notebookId" Decode.int)
        (Decode.field "title" Decode.string)


notebookHistoryTitleEncoder : { title : String } -> Encode.Value
notebookHistoryTitleEncoder notebookHistoryTitle =
    Encode.object
        [ ("title", Encode.string notebookHistoryTitle.title) ]
