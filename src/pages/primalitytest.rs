use seed::prelude::*;
use crate::Msg;

pub fn render() -> seed::dom_types::Node<Msg> {
    div![
        h1!["Primality Test, Calculate Primes"],
        br![],
        br![],
        br![],
        //TODO: Convert the j.js file into rust!
        Script!(attrs!{At::Src => "https://static.bigprimes.net/j.js"}),
        El::from_html("
            <table cellpadding=\"10\" cellspacing=\"0\" class=\"text\" width=\"200\" style=\"border:1px solid #444; background-color:#e0faed\">
            <tr>
            <td>
            <form name=\"primetest\" onsubmit=\"return false\" action=\"\">
            This test uses javascript and is limited to checking numbers upto 16 digits.<br /><br />
            Is <input type=\"text\" size=\"16\" name=\"input\" value=\"\" maxlength=\"16\" /> prime? 
            <input onclick=\"check(false,0)\" type=\"button\" value=\"Check!\" />
            <br /><br />
            <textarea name=\"javascriptoutput\" cols=\"40\" rows=\"2\" disabled=\"disabled\"></textarea>
            </form>
            </td>
            </tr>
            </table>
            <br /><br />
            <table bgcolor=\"#e0faed\" cellpadding=\"10\" cellspacing=\"0\" class=\"text\" width=\"200\" style=\"border:1px solid #444;\">
            <tr>
            <td>
            <form name=\"primelist\" onsubmit=\"return false\" action=\"\">
            This uses javascript and is limited to checking numbers upto 15 digits.<br /><br />
            This will show <input size=\"4\" name=\"primes\" maxlength=\"2\" value=\"1\" />
            prime numbers after <input size=\"16\" name=\"start\" maxlength=\"15\" value=\"0\" /> 
            <input onclick=\"primelist.javascriptlistoutput.value='';listy();\" type=\"button\" value=\"Go!\" />
            <br /><br />
            <textarea id=\"javascriptlistoutput\" cols=\"60\" rows=\"10\" disabled=\"disabled\"></textarea>
            </form>
            </td>
            </tr>
            </table>
        "),        
    ]
}
