module Model exposing (..)

import Page exposing (Page(..))


type alias Model =
    { page : Page
    , uid : String
    }


new : Model
new =
    { page = Home
    , uid = ""
    }


updateUid : Model -> String -> Model
updateUid model uid =
    { model | uid = uid }
