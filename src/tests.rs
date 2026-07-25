#[cfg(test)]
mod tests
{
    use super::*;
    // for styling
    use ratatui::style::Style;

    #[test]
    // check if app renders correctly
    fn render()
    {
        // create a default app and buffer
        let app = App::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
        // render the buffer area
        app.render(buf.area, &mut buf);
        // compare rendered buffer to expected
        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        // create new title style and set as bold
        let title_style = Style::new().bold();
        // create new counter style and set to yellow color
        let counter_style = Style::new().yellow();
        // create new key style and set to blue and bold
        let key_style = Style::new().blue().bold();
        // set the styles for expected
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);
        // assert that the buffer and expected are equal
        assert_eq!(buf, expected);
    }

    #[test]
    // test that key events are handled correctly
    fn handle_key_event()
    {
        // create a default app
        let mut app = App::default();
        // test right key event
        app.handle_key_event(KeyCode::Right.into());
        // assert that counter is incremented
        assert_eq!(app.counter, 1);

        // test left key event
        app.handle_key_event(KeyCode::Left.into());
        // assert that counter is decremented
        assert_eq!(app.counter, 0);

        // shadowing by creating new app with same variable name
        let mut app = App::default();
        // test quit key event
        app.handle_key_event(KeyCode::Char('q').into());
        // assert that exit flag is set
        assert!(app.exit);
    }
}
