import { combineReducers } from "redux";
import sites from "./sites";
import posts from "./posts";
import { object } from "prop-types";

export default combineReducers({
    posts,
    sites
})