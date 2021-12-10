module Flexy.UI
import Flexy.Layout.*

public native func RenderElem(root: ref<Elem>, size: Vector2) -> ref<inkWidget>;

public func RenderElem(root: ref<Elem>) -> ref<inkWidget> {
  return RenderElem(root, new Vector2(0, 0));
}

public abstract class Elem {
  let layout: ref<Layout>;

  public func GetChildren() -> array<ref<Elem>> = [];
  public func GetLayout() -> ref<Layout> = this.layout;
  public func GetPreferredSize() -> Vector2 = new Vector2(0, 0);

  public func Layout(layout: ref<Layout>) -> ref<Elem> {
    this.layout = layout;
    return this;
  }

  public func Render(pos: Vector2, size: Vector2) -> ref<inkWidget>;
}

public class Box extends Elem {
  let children: array<ref<Elem>>;
  let backgroundColor: Color;
  let hasBackground: Bool;

  public static func New(children: array<ref<Elem>>) -> ref<Box> {
    let self = new Box();
    self.children = children;
    self.layout = new Layout().Padding(5);
    return self;
  }

  public static func New() -> ref<Box> {
    return Box.New([]);
  }

  public func BackgroundColor(color: Color) -> ref<Box> {
    this.backgroundColor = color;
    this.hasBackground = true;
    return this;
  }

  public func Child(child: ref<Elem>) -> ref<Box> {
    ArrayPush(this.children, child);
    return this;
  }

  public func GetChildren() -> array<ref<Elem>> = this.children;

  public func Render(pos: Vector2, size: Vector2) -> ref<inkWidget> {
    let canv = new inkCanvas();
    canv.SetTranslation(pos);
    canv.SetSize(size);

    if this.hasBackground {
      let rect = new inkRectangle();
      rect.SetTintColor(this.backgroundColor);
      rect.SetSize(size);
      canv.AddChildWidget(rect);
    }
    
    return canv;
  }
}

public class Text extends Elem {
  let text: String;
  let fontSize: Int32;
  let color: Color;

  public static func New(text: String) -> ref<Text> {
    let self = new Text();
    self.text = text;
    self.fontSize = 24;
    self.color = new Color(Cast(255), Cast(255), Cast(255), Cast(255));
    self.layout = Layout.New();
    return self;
  }

  public func Color(color: Color) -> ref<Text> {
    this.color = color;
    return this;
  }

  public func FontSize(size: Int32) -> ref<Text> {
    this.fontSize = size;
    return this;
  }

  public func Render(pos: Vector2, size: Vector2) -> ref<inkWidget> {
    let text = new inkText();
    text.SetTranslation(pos);
    text.SetFontFamily("base\\gameplay\\gui\\fonts\\raj\\raj.inkfontfamily");
    text.SetText(this.text);
    text.SetFontSize(this.fontSize);
    text.SetTintColor(this.color);
    return text;
  }
}

public class Image extends Elem {
  let atlas: ResRef;
  let texturePart: String;
  let nineSliceScale: Bool;
  let tint: Color;

  public static func New(atlas: String) -> ref<Image> {
    let self = new Image();
    self.atlas = ResRef.FromString(atlas);
    return self;
  }

  public func TexturePart(part: String) -> ref<Image> {
    this.texturePart = part;
    return this;
  }

  public func NineSliceScale(val: Bool) -> ref<Image> {
    this.nineSliceScale = val;
    return this;
  }

  public func Tint(color: Color) -> ref<Image> {
    this.tint = color;
    return this;
  }

  public func Render(pos: Vector2, size: Vector2) -> ref<inkWidget> {
    let img = new inkImage();
    img.SetTranslation(pos);
    img.SetSize(size);

    img.SetAtlasResource(this.atlas);
    if NotEquals(this.texturePart, "") {
      img.SetTexturePart(StringToName(this.texturePart));
    }
    img.SetNineSliceScale(this.nineSliceScale);
    img.SetTintColor(this.tint);
    return img;
  }
}
