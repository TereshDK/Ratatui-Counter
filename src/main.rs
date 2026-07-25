// comments by TereshDK

// for input / output
use std::io::Result;
// for terminal events
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
// for ratatui
use ratatui::
{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal,
    Frame,
};

/* #[derive(...)] is an attribute that implements the Debug
* and Default traits for this App struct
* derive implements the trait explicit implementation like impl Debug for App
* Debug trait allows printing of struct for debugging with {:?}
* Default trait automatically creates a default constructor
*/
#[derive(Debug, Default)]
pub struct App
{
    counter:i32,
    exit:bool,
}

// entry point
// Result<()> is for recoverable error handling and is used by ratatui::run
fn main() -> Result<()>
{
    // run the application
    // ratatui::run calls ratatui::init and ratatui::restore that creates and runs the App
    // .run result is not returned until after terminal is restored

    /*
    * closure (anonymous function)
    * intead of writing:
    * fn start(terminal: &mut Terminal) -> Result<()>
    * {
    *     App::default().run(terminal)
    * }
    * Rust allows it to be written as |terminal| App::default().run(terminal)
    * a closure is like a lambda in other languages but
    * can also capture variables from the surrounding scope
    */
    ratatui::run(|terminal| App::default().run(terminal))
}

// construct for App
impl App
{
    // runs the application's main loop until user quits
    pub fn run(&mut self, terminal:&mut DefaultTerminal) -> Result<()>
    {
        // loop until exit flag is set
        while !self.exit
        {
            // draw a single frame
            // ? operator is for Result<> matching with out writing match
            terminal.draw(|frame| self.draw(frame))?;
            // handle user input events like pressing a key
            self.handle_events()?;
        }
        // return success
        Ok(())
    }

    // render the app as a widget
    fn draw(&self, frame:&mut Frame)
    {
        // render the frame area as a widget
        frame.render_widget(self, frame.area());
    }

    // updates application's state based on user input
    fn handle_events(&mut self) -> Result<()>
    {
        // blocks until there is an event
        match event::read()?
        {
            // checks if a key of type Event::Key is a press event
            // it's important to check that the event is a key press event
            // as crossterm also emits key release and repeat events on Windows
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press =>
            {
                // handles the key event
                self.handle_key_event(key_event)
            }
            // ignores other events
            _ => {}
        }
        // return success
        Ok(())
    }

    // handles key events
    fn handle_key_event(&mut self, key_event:KeyEvent)
    {
        // match the key event code to determine which action to take
        match key_event.code
        {
            // exit the application if 'q' is pressed
            KeyCode::Char('q') => self.exit(),
            // decrement the counter if the left arrow is pressed
            KeyCode::Left => self.decrement_counter(),
            // increment the counter if the right arrow is pressed
            KeyCode::Right => self.increment_counter(),
            // ignore other key events
            _ => {}
        }
    }

    // exits the application
    fn exit(&mut self)
    {
        // set the exit flag to true
        self.exit = true
    }

    // increments the counter
    fn increment_counter(&mut self)
    {
        // increment the counter by 1
        self.counter += 1;
    }

    // decrements the counter
    fn decrement_counter(&mut self)
    {
        // decrement the counter by 1
        self.counter -= 1;
    }
}

// construct for Widget
// &App used as render function will not mutate any state
// and will need to use app after call to draw
impl Widget for &App
{
    // render the widget
    fn render(self, rect_area:Rect, buffer:&mut Buffer)
    {
        // create a new title for the app widget
        let title = Line::from("Ratatui Counter".bold());
        // create a new line for the instructions
        let instructions = Line::from(vec!
        [
            " Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);
        // create a block that positions the title and instructions
        let block = Block::bordered()
            // center title
            .title(title.centered())
            // position instructions below title and center
            .title_bottom(instructions.centered())
            // set border style to thick
            .border_set(border::THICK);
        // counter text to display the current value to display state
        let counter_text = Text::from(vec!
        [
            Line::from(vec!
            [
                // label for counter value
                "Value: ".into(),
                // convert counter value to string and color it yellow
                self.counter.to_string().yellow(),
            ])
        ]);
        // render a paragraph widget with application's state in block
        // block and paragraph will take up entire widget size
        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(rect_area, buffer);
    }
}
