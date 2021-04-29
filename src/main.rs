mod ast;
mod util;

use ast::lexer::Lexer;

fn main() {
    let mut lexer: Lexer = Lexer::new(
        &"co = coroutine.create(
    function(i)
        print(i);
    end
)

coroutine.resume(co, 1)
print(coroutine.status(co))


co = coroutine.wrap(
    function(i)
        print(i);
    end
)

co(1)

co2 = coroutine.create(
    function()
        for i=1,10 do
            print(i)
            if i == 3 then
                print(coroutine.status(co2))
                print(coroutine.running())
            end
            coroutine.yield()
        end
    end
)

coroutine.resume(co2)
coroutine.resume(co2)
coroutine.resume(co2)

print(coroutine.status(co2))
print(coroutine.running())

function whiteip()
    if next(ipWhitelist) ~= nil then
        for _,ip in pairs(ipWhitelist) do
            if getClientIp()==ip then
                return true
            end
        end
    end
        return false
end

function blockip()
     if next(ipBlocklist) ~= nil then
         for _,ip in pairs(ipBlocklist) do
             if getClientIp()==ip then
                 ngx.exit(403)
                 return true
             end
         end
     end
         return false
end
        ".to_string());
    for token in lexer {
        println!("{}", token);
    }
}
