(ns problema21.core)

(defn combinations [l]
  (for [x (get l 0) y(get l 1) z(get l 2) w(get l 3)] (vector x y z w)))

(defn runmatch [boss me]
  (def bh (get boss :h))
  (def mh (get me :h))
  (if (<= bh 0) (get me :c)
      (if (<= mh 0) -1
          (runmatch
           {:h (- bh (- (get me :dmg) (get boss :a))), :dmg (get boss :dmg), :a (get boss :a)}; Boss
           {:h (- mh (- (get boss :dmg) (get me :a))), :dmg (get me :dmg), :a (get me :a), :c (get me :c)}; Me
           ))))

(defn runmatch_p2 [boss me]
  (def bh (get boss :h))
  (def mh (get me :h))
  (if (<= bh 0) -1 
      (if (<= mh 0) (get me :c) 
          (runmatch_p2
           {:h (- bh (- (get me :dmg) (get boss :a))), :dmg (get boss :dmg), :a (get boss :a)}; Boss
           {:h (- mh (- (get boss :dmg) (get me :a))), :dmg (get me :dmg), :a (get me :a), :c (get me :c)}; Me
           ))))

(defn rungame [l]
  (for [build l]
    (do
      (def boss {:h 103, :dmg 9, :a 2})
      (def me {:h 100,
               :dmg (reduce + (for [i build] (get i :damage))),
               :a (reduce + (for [i build] (get i :armor))),
               :c (reduce + (for [i build] (get i :cost)))})
                                        ;(runmatch boss me)
      (runmatch_p2 boss me)
      )))


(defn main
  (do
    (def weapon [{:name "Dagger", :cost 8, :damage 4, :armor 0}
                 {:name "Shortsword", :cost 10, :damage 5, :armor 0}
                 {:name "Warhammer", :cost 25, :damage 6, :armor 0}
                 {:name "Longsword", :cost 40, :damage 7, :armor 0}
                 {:name "Greataxe", :cost 74, :damage 8, :armor 0}])

    (def armor [{:name "Leather", :cost 13, :damage 0, :armor 1}
                {:name "Chainmail", :cost 31, :damage 0, :armor 2}
                {:name "Splintmail", :cost 53, :damage 0, :armor 3}
                {:name "Bandedmail", :cost 75, :damage 0, :armor 4}
                {:name "Platemail", :cost 102, :damage 0, :armor 5}])

    (def rings [{:name "Damage +1", :cost 25,  :damage 1, :armor 0}
                {:name "Damage +2", :cost 50,  :damage 2, :armor 0}
                {:name "Damage +3", :cost 100, :damage 3, :armor 0}
                {:name "Defense +1", :cost 20, :damage 0, :armor 1}
                {:name "Defense +2", :cost 40, :damage 0, :armor 2}
                {:name "Defense +3", :cost 80, :damage 0, :armor 3}])
    (def x (vec (combinations [weapon armor rings rings])))
    (def res (vec (rungame x)))
    (sort(filter (fn [x] (> x 0)) res))))
