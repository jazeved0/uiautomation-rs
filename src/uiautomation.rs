use std::ptr::null_mut;

use windows::Win32::Foundation::BSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::Foundation::POINT;
use windows::Win32::Foundation::RECT;
use windows::Win32::System::Com::CLSCTX_ALL;
use windows::Win32::System::Com::COINIT_MULTITHREADED;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::CoInitializeEx;
use windows::Win32::UI::Accessibility::CUIAutomation;
use windows::Win32::UI::Accessibility::IUIAutomation;
use windows::Win32::UI::Accessibility::IUIAutomationElement;
use windows::Win32::UI::Accessibility::IUIAutomationElementArray;
use windows::Win32::UI::Accessibility::IUIAutomationTreeWalker;
use windows::Win32::UI::Accessibility::OrientationType;

use crate::errors::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone)]
pub struct UIAutomation {
    automation: IUIAutomation
}

impl UIAutomation {
    pub fn new() -> Result<UIAutomation> {
        let automation: IUIAutomation;
        unsafe {
            CoInitializeEx(null_mut(), COINIT_MULTITHREADED)?;
            automation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        }

        Ok(UIAutomation {
            automation
        })
    }

    pub fn compare_elements(&self, element1: &UIElement, element2: &UIElement) -> Result<bool> {
        let same;
        unsafe {
            same = self.automation.CompareElements(element1.as_ref(), element2.as_ref())?;
        }
        Ok(same.as_bool())
    }

    pub fn element_from_handle(&self, hwnd: HWND) -> Result<UIElement> {
        let element = unsafe {
            self.automation.ElementFromHandle(hwnd)?
        };

        Ok(UIElement::from(element))
    }

    pub fn element_from_point(&self, point: POINT) -> Result<UIElement> {
        let element = unsafe {
            self.automation.ElementFromPoint(point)?
        };

        Ok(UIElement::from(element))
    }

    pub fn get_focused_element(&self) -> Result<UIElement> {
        let element = unsafe {
            self.automation.GetFocusedElement()?
        };

        Ok(UIElement::from(element))
    }

    pub fn get_root_element(&self) -> Result<UIElement> {
        let element: IUIAutomationElement;
        unsafe {
            element = self.automation.GetRootElement()?;
        }

        Ok(UIElement::from(element))
    }

    pub fn create_tree_walker(&self) -> Result<UITreeWalker> {
        let tree_walker: IUIAutomationTreeWalker;
        unsafe {
            let condition = self.automation.CreateTrueCondition()?;
            tree_walker = self.automation.CreateTreeWalker(condition)?;
        }

        Ok(UITreeWalker::from(tree_walker))
    }

    pub fn create_matcher(&self) -> UIMatcher {
        UIMatcher::new(self.clone())
    }
}

impl From<IUIAutomation> for UIAutomation {
    fn from(automation: IUIAutomation) -> Self {
        UIAutomation {
            automation
        }
    }
}

impl Into<IUIAutomation> for UIAutomation {
    fn into(self) -> IUIAutomation {
        self.automation
    }
}

impl AsRef<IUIAutomation> for UIAutomation {
    fn as_ref(&self) -> &IUIAutomation {
        &self.automation
    }
}

#[derive(Clone)]
pub struct UIElement {
    element: IUIAutomationElement
}

impl UIElement {
    pub fn get_name(&self) -> Result<String> {
        let name: BSTR;
        unsafe {
            name = self.element.CurrentName()?;
        }

        Ok(name.to_string())
    }

    pub fn get_automation_id(&self) -> Result<String> {
        let automation_id = unsafe {
            self.element.CurrentAutomationId()?
        };

        Ok(automation_id.to_string())
    }

    pub fn get_process_id(&self) -> Result<i32> {
        let id = unsafe {
            self.element.CurrentProcessId()?
        };

        Ok(id)
    }

   pub fn get_classname(&self) -> Result<String> {
        let classname: BSTR;
        unsafe {
            classname = self.element.CurrentClassName()?;
        }

        Ok(classname.to_string())
    }

    pub fn get_control_type(&self) -> Result<i32> {
        let control_type = unsafe {
            self.element.CurrentControlType()?
        };

        Ok(control_type)
    }

    pub fn get_localized_control_type(&self) -> Result<String> {
        let control_type = unsafe {
            self.element.CurrentLocalizedControlType()?
        };

        Ok(control_type.to_string())
    }

    pub fn get_accelerator_key(&self) -> Result<String> {
        let accelerator_key = unsafe {
            self.element.CurrentAcceleratorKey()?
        };

        Ok(accelerator_key.to_string())
    }

    pub fn get_access_key(&self) -> Result<String> {
        let access_key = unsafe {
            self.element.CurrentAccessKey()?
        };

        Ok(access_key.to_string())
    }

    pub fn has_keyboard_focus(&self) -> Result<bool> {
        let has_focus = unsafe {
            self.element.CurrentHasKeyboardFocus()?
        };

        Ok(has_focus.as_bool())
    }

    pub fn is_keyboard_focusable(&self) -> Result<bool> {
        let focusable = unsafe {
            self.element.CurrentIsKeyboardFocusable()?
        };

        Ok(focusable.as_bool())
    }

    pub fn is_enabled(&self) -> Result<bool> {
        let enabled = unsafe {
            self.element.CurrentIsEnabled()?
        };

        Ok(enabled.as_bool())
    }

    pub fn get_help_text(&self) -> Result<String> {
        let text = unsafe {
            self.element.CurrentHelpText()?
        };

        Ok(text.to_string())
    }

    pub fn get_culture(&self) -> Result<i32> {
        let culture = unsafe {
            self.element.CurrentCulture()?
        };

        Ok(culture)
    }

    pub fn is_control_element(&self) -> Result<bool> {
        let is_control = unsafe {
            self.element.CurrentIsControlElement()?
        };

        Ok(is_control.as_bool())
    }

    pub fn is_content_element(&self) -> Result<bool> {
        let is_content = unsafe {
            self.element.CurrentIsContentElement()?
        };

        Ok(is_content.as_bool())
    }

    pub fn is_password(&self) -> Result<bool> {
        let is_password = unsafe {
            self.element.CurrentIsPassword()?
        };

        Ok(is_password.as_bool())
    }

    pub fn get_native_window_handle(&self) -> Result<HWND> {
        let handle = unsafe {
            self.element.CurrentNativeWindowHandle()?
        };

        Ok(handle)
    }

    pub fn get_item_type(&self) -> Result<String> {
        let item_type = unsafe {
            self.element.CurrentItemType()?
        };

        Ok(item_type.to_string())
    }

    pub fn is_off_screen(&self) -> Result<bool> {
        let off_screen = unsafe {
            self.element.CurrentIsOffscreen()?
        };

        Ok(off_screen.as_bool())
    }

    pub fn get_orientation_type(&self) -> Result<OrientationType> {
        let orientation = unsafe {
            self.element.CurrentOrientation()?
        };

        Ok(orientation)
    }

    pub fn get_framework_id(&self) -> Result<String> {
        let id = unsafe {
            self.element.CurrentFrameworkId()?
        };

        Ok(id.to_string())
    }

    pub fn is_required_for_form(&self) -> Result<bool> {
        let required = unsafe {
            self.element.CurrentIsRequiredForForm()?
        };

        Ok(required.as_bool())
    }

    pub fn is_data_valid_for_form(&self) -> Result<bool> {
        let valid = unsafe {
            self.element.CurrentIsDataValidForForm()?
        };

        Ok(valid.as_bool())
    }

    pub fn get_item_status(&self) -> Result<String> {
        let status = unsafe {
            self.element.CurrentItemStatus()?
        };

        Ok(status.to_string())
    }

    pub fn get_bounding_rectangle(&self) -> Result<RECT> {
        let rect = unsafe {
            self.element.CurrentBoundingRectangle()?
        };

        Ok(rect)
    }

    pub fn get_labeled_by(&self) -> Result<UIElement> {
        let labeled_by = unsafe {
            self.element.CurrentLabeledBy()?
        };

        Ok(UIElement::from(labeled_by))
    }

    pub fn get_controller_for(&self) -> Result<Vec<UIElement>> {
        let elements = unsafe {
            self.element.CurrentControllerFor()?
        };

        to_elements(elements)
    }

    pub fn get_described_by(&self) -> Result<Vec<UIElement>> {
        let elements = unsafe {
            self.element.CurrentDescribedBy()?
        };

        to_elements(elements)
    }

    pub fn get_flows_to(&self) -> Result<Vec<UIElement>> {
        let elements = unsafe {
            self.element.CurrentFlowsTo()?
        };

        to_elements(elements)
    }

    pub fn get_provider_description(&self) -> Result<String> {
        let descr = unsafe {
            self.element.CurrentProviderDescription()?
        };

        Ok(descr.to_string())
    }

    pub fn set_focus(&self) -> Result<()> {
        unsafe {
            self.element.SetFocus()?;
        }

        Ok(())
    }
}

fn to_elements(elements: IUIAutomationElementArray) -> Result<Vec<UIElement>> {
    let mut arr: Vec<UIElement> = Vec::new();
    unsafe {
        for i in 0..elements.Length()? {
            let elem = UIElement::from(elements.GetElement(i)?);
            arr.push(elem);
        }
    }

    Ok(arr)

}

impl From<IUIAutomationElement> for UIElement {
    fn from(element: IUIAutomationElement) -> Self {
        UIElement {
            element
        }
    }
}

impl Into<IUIAutomationElement> for UIElement {
    fn into(self) -> IUIAutomationElement {
        self.element
    }
}

impl AsRef<IUIAutomationElement> for UIElement {
    fn as_ref(&self) -> &IUIAutomationElement {
        &self.element
    }
}

#[derive(Clone)]
pub struct UITreeWalker {
    tree_walker: IUIAutomationTreeWalker
}

impl UITreeWalker {
    pub fn get_parent(&self, element: &UIElement) -> Result<UIElement> {
        let parent: IUIAutomationElement;
        unsafe {
            parent = self.tree_walker.GetParentElement(&element.element)?;
        }

        Ok(UIElement::from(parent))
    }

    pub fn get_first_child(&self, element: &UIElement) -> Result<UIElement> {
        let child: IUIAutomationElement;
        unsafe {
            child = self.tree_walker.GetFirstChildElement(&element.element)?;
        }

        Ok(UIElement::from(child))
    }

    pub fn get_last_child(&self, element: &UIElement) -> Result<UIElement> {
        let child: IUIAutomationElement;
        unsafe {
            child = self.tree_walker.GetLastChildElement(&element.element)?;
        }

        Ok(UIElement::from(child))
    }

    pub fn get_next_sibling(&self, element: &UIElement) -> Result<UIElement> {
        let sibling: IUIAutomationElement;
        unsafe {
            sibling = self.tree_walker.GetNextSiblingElement(&element.element)?;
        }

        Ok(UIElement::from(sibling))
    }

    pub fn get_previous_sibling(&self, element: &UIElement) -> Result<UIElement> {
        let sibling: IUIAutomationElement;
        unsafe {
            sibling = self.tree_walker.GetPreviousSiblingElement(&element.element)?;
        }

        Ok(UIElement::from(sibling))
    }
}

impl From<IUIAutomationTreeWalker> for UITreeWalker {
    fn from(tree_walker: IUIAutomationTreeWalker) -> Self {
        UITreeWalker {
            tree_walker
        }
    }
}

impl Into<IUIAutomationTreeWalker> for UITreeWalker {
    fn into(self) -> IUIAutomationTreeWalker {
        self.tree_walker
    }
}

impl AsRef<IUIAutomationTreeWalker> for UITreeWalker {
    fn as_ref(&self) -> &IUIAutomationTreeWalker {
        &self.tree_walker
    }
}

pub struct UIMatcher {
    automation: UIAutomation,
    depth: u32,
    from: Option<UIElement>,
    condition: Option<Box<dyn Condition>>
}

impl UIMatcher {
    pub fn new(automation: UIAutomation) -> Self {
        UIMatcher {
            automation,
            depth: 5,
            from: None,
            condition: None
        }
    }

    pub fn from(mut self, element: UIElement) -> Self {
        self.from = Some(element);
        self
    }

    pub fn depth(mut self, depth: u32) -> Self {
        self.depth = depth;
        self
    }

    pub fn filter(mut self, condition: Box<dyn Condition>) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn contains_name<S: Into<String>>(self, name: S) -> Self {
        let condition = NameCondition {
            value: name.into(),
            casesensitive: false,
            partial: true
        };
        self.filter(Box::new(condition))
    }

    pub fn match_name<S: Into<String>>(self, name: S) -> Self {
        let condition = NameCondition {
            value: name.into(),
            casesensitive: false,
            partial: false
        };
        self.filter(Box::new(condition))
    }

    pub fn find_first(&self) -> Result<UIElement> {
        // let root = if let Some(ref from) = self.from {
        //     from.clone()
        // } else {
        //     self.automation.get_root_element()?
        // };
        // let walker = self.automation.create_tree_walker()?;
        let (root, walker) = self.prepare()?;

        let mut elements: Vec<UIElement> = Vec::new();
        self.search(&walker, &root, &mut elements, 1, true)?;

        if elements.is_empty() {
            Err(Error::from("NOTFOUND"))
        } else {
            Ok(elements.remove(0))
        }
    }

    pub fn find_all(&self) -> Result<Vec<UIElement>> {
        let (root, walker) = self.prepare()?;

        let mut elements: Vec<UIElement> = Vec::new();
        self.search(&walker, &root, &mut elements, 1, false)?;

        if elements.is_empty() {
            Err(Error::from("NOTFOUND"))
        } else {
            Ok(elements)
        }
    }

    fn prepare(&self) -> Result<(UIElement, UITreeWalker)> {
        let root = if let Some(ref from) = self.from {
            from.clone()
        } else {
            self.automation.get_root_element()?
        };
        let walker = self.automation.create_tree_walker()?;
        
        Ok((root, walker))
    }

    fn search(&self, walker: &UITreeWalker, element: &UIElement, elements: &mut Vec<UIElement>, depth: u32, first_only: bool) -> Result<()> {
        if self.is_matched(element)? {
            elements.push(element.clone());

            if first_only {
                return Ok(());
            }
        }

        if depth < self.depth {
            let mut next = walker.get_first_child(element);
            while let Ok(ref child) = next {
                self.search(walker, child, elements, depth + 1, first_only)?;
                if first_only && !elements.is_empty() {
                    return Ok(());
                }

                next = walker.get_next_sibling(child);
            }
        }

        Ok(())
    }

    fn is_matched(&self, element: &UIElement) -> Result<bool> {
        if let Some(ref condition) = self.condition {
            condition.judge(element)
        } else {
            Ok(true)
        }
    }
}

pub trait Condition {
    fn judge(&self, element: &UIElement) -> Result<bool>;
}

struct NameCondition {
    value: String,
    casesensitive: bool,
    partial: bool
}

impl Condition for NameCondition {
    fn judge(&self, element: &UIElement) -> Result<bool> {
        let element_name = element.get_name()?;
        let element_name = element_name.as_str();
        let condition_name = self.value.as_str();

        Ok(
            if self.partial {
                if self.casesensitive {
                    element_name.contains(condition_name)
                } else {
                    let element_name = element_name.to_lowercase();
                    let condition_name = condition_name.to_lowercase();

                    element_name.contains(&condition_name)
                }
            } else {
                if self.casesensitive {
                    element_name == condition_name
                } else {
                    element_name.eq_ignore_ascii_case(condition_name)
                }
            }
        )
    }
}