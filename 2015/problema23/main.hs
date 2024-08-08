instruction :: String -> IO ()
instruction "hlf" = putStrLn "hlf"
instruction "jio" = putStrLn "jio"
instruction x = putStrLn "Non recog"

main :: IO ()
main = do
  instruction "hlf"
  instruction "jio"
  instruction "a"
