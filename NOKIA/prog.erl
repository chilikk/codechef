-module(prog).
-export([main/0]).

fmin(0) -> 0;
fmin(X) -> X2 = X div 2, X+1 + fmin(X2) + fmin(X-X2-1).
fmax(X) -> (X+2)*(X+1) div 2 - 1.

test(0) -> ok;
test(T) -> do_test(), test(T-1).

do_test() ->
    {ok, [N,M]} = io:fread([],"~d ~d"),
    case fmax(N) of
        X when X<M ->
            io:format("~B~n", [M-X]);
        _ ->
            case fmin(N) of
                X when X>M ->
                    io:format("-1~n");
                _ ->
                    io:format("0~n")
            end
    end.

main() ->
    try
        case io:fread([],"~d") of
            {ok,[T]} -> test(T);
            _ -> ok
        end
    catch _:_ ->
        ok
    end.
