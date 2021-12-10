module Flexy.Markup
import Flexy.UI.Elem

public native func ParseElem(str: String) -> ref<Elem>;
public native func LoadElem(name: String) -> ref<Elem>;
