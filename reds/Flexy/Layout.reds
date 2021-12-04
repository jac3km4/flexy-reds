module Flexy.Layout

public class Layout {
  let flexDirection: FlexDirection;
  let flexWrap: FlexWrap;
  let alignContent: FlexAlign;
  let justifyContent: FlexAlign;
  let height: ref<Dim>;
  let width: ref<Dim>;
  let left: ref<Dim>;
  let right: ref<Dim>;
  let top: ref<Dim>;
  let bottom: ref<Dim>;
  let marginLeft: Float;
  let marginRight: Float;
  let marginTop: Float;
  let marginBottom: Float;
  let paddingLeft: Float;
  let paddingRight: Float;
  let paddingTop: Float;
  let paddingBottom: Float;
  let flexGrow: Float;

  public static func New() -> ref<Layout> = new Layout();

  public func GetFlexDirection() -> FlexDirection = this.flexDirection;
  public func GetFlexWrap() -> FlexWrap = this.flexWrap;
  public func GetAlignContent() -> FlexAlign = this.alignContent;
  public func GetJustifyContent() -> FlexAlign = this.justifyContent;
  public func GetHeight() -> ref<Dim> = this.height;
  public func GetWidth() -> ref<Dim> = this.width;
  public func GetMarginLeft() -> Float = this.marginLeft;
  public func GetMarginRight() -> Float = this.marginRight;
  public func GetMarginTop() -> Float = this.marginTop;
  public func GetMarginBottom() -> Float = this.marginBottom;
  public func GetPaddingLeft() -> Float = this.paddingLeft;
  public func GetPaddingRight() -> Float = this.paddingRight;
  public func GetPaddingTop() -> Float = this.paddingTop;
  public func GetPaddingBottom() -> Float = this.paddingBottom;
  public func GetFlexGrow() -> Float = this.flexGrow;

  public func FlexDirection(dir: FlexDirection) -> ref<Layout> {
    this.flexDirection = dir;
    return this;
  }

  public func FlexWrap(wrap: FlexWrap) -> ref<Layout> {
    this.flexWrap = wrap;
    return this;
  }

  public func JustifyContent(justify: FlexAlign) -> ref<Layout> {
    this.justifyContent = justify;
    return this;
  }

  public func AlignContent(align: FlexAlign) -> ref<Layout> {
    this.alignContent = align;
    return this;
  }

  public func Width(str: String) -> ref<Layout> {
    this.width = ParseDim(str);
    return this;
  }

  public func Height(str: String) -> ref<Layout> {
    this.height = ParseDim(str);
    return this;
  }

  public func Margin(value: Float) -> ref<Layout> {
    this.marginLeft = value;
    this.marginRight = value;
    this.marginTop = value;
    this.marginBottom = value;
    return this;
  }

  public func Padding(value: Float) -> ref<Layout> {
    this.paddingLeft = value;
    this.paddingRight = value;
    this.paddingTop = value;
    this.paddingBottom = value;
    return this;
  }

  public func FlexGrow(value: Float) -> ref<Layout> {
    this.flexGrow = value;
    return this;
  }
}

public class Dim {
  let value: Float;
  let unit: Unit;

  public static func New(value: Float, unit: Unit) -> ref<Dim> {
    let self = new Dim();
    self.value = value;
    self.unit = unit;
    return self;
  }

  public func GetValue() -> Float = this.value;
  public func GetUnit() -> Unit = this.unit;
}

enum Unit {
  Auto = 0,
  Point = 1,
  Percent = 2
}

enum FlexDirection {
  Row = 0,
  Column = 1,
  RowReverse = 2,
  ColumnReverse = 3
}

enum FlexWrap {
  NoWrap = 0,
  Wrap = 1,
  WrapReverse = 2
}

enum FlexAlign {
  Inherit = 0,
  Stretch = 1,
  Start = 2,
  Center = 3,
  End = 3,
  SpaceBetween = 5,
  SpaceAround = 6,
  Baseline = 7
}

func NewDim(val: Float, unit: Unit) -> ref<Dim> {
  return Dim.New(val, unit);
}

native func ParseDim(str: String) -> ref<Dim>;
