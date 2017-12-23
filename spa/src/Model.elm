module Model exposing (..)

import Page exposing (Page(..))


type alias Model =
    { page: Page }


new : Model
new =
    { page = Home }
