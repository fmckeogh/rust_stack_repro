#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use AArch64_NextTableBase::*;
use AArch64_S1ApplyTablePerms::*;
use common::*;
pub fn AArch64_S1NextWalkStateTable<T: Tracer>(
    state: &mut State,
    tracer: &T,
    currentstate: ProductType96e7acababe246a1,
    s2fs1mro: bool,
    regime: u32,
    walkparams: ProductTypeef284266e139aee2,
    descriptor: Bits,
) -> ProductType96e7acababe246a1 {
    #[derive(Default)]
    struct FunctionState {
        tablebase: ProductTypeda0231e9dc169f81,
        ga_13661: ProductTypeda0231e9dc169f81,
        nextstate: ProductType96e7acababe246a1,
        gs_18209: bool,
        ga_13649: ProductTypeda0231e9dc169f81,
        nstable: bool,
        protectedbit: bool,
        gs_18185: bool,
        gs_18208: bool,
        currentstate: ProductType96e7acababe246a1,
        s2fs1mro: bool,
        regime: u32,
        walkparams: ProductTypeef284266e139aee2,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        currentstate,
        s2fs1mro,
        regime,
        walkparams,
        descriptor,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_0_0: read-var walkparams.3:struct
        let s_0_0: bool = fn_state.walkparams._3;
        // D s_0_1: read-var walkparams.7:struct
        let s_0_1: bool = fn_state.walkparams._7;
        // D s_0_2: read-var walkparams.36:struct
        let s_0_2: u32 = fn_state.walkparams._36;
        // D s_0_3: read-var descriptor:bv
        let s_0_3: Bits = fn_state.descriptor;
        // D s_0_4: call AArch64_NextTableBase(s_0_3, s_0_0, s_0_1, s_0_2)
        let s_0_4: u64 = AArch64_NextTableBase(
            state,
            tracer,
            s_0_3,
            s_0_0,
            s_0_1,
            s_0_2,
        );
        // D s_0_5: write-var tablebase.0 <= s_0_4
        fn_state.tablebase._0 = s_0_4;
        // D s_0_6: read-var currentstate.0:struct
        let s_0_6: ProductTypeda0231e9dc169f81 = fn_state.currentstate._0;
        // D s_0_7: write-var ga#13649 <= s_0_6
        fn_state.ga_13649 = s_0_6;
        // D s_0_8: read-var ga#13649.1:struct
        let s_0_8: u32 = fn_state.ga_13649._1;
        // C s_0_9: const #1u : u32
        let s_0_9: u32 = 1;
        // D s_0_10: cmp-eq s_0_8 s_0_9
        let s_0_10: bool = ((s_0_8) == (s_0_9));
        // N s_0_11: branch s_0_10 b30 b1
        if s_0_10 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_1_0: read-var currentstate.0:struct
        let s_1_0: ProductTypeda0231e9dc169f81 = fn_state.currentstate._0;
        // D s_1_1: write-var ga#13661 <= s_1_0
        fn_state.ga_13661 = s_1_0;
        // D s_1_2: read-var ga#13661.1:struct
        let s_1_2: u32 = fn_state.ga_13661._1;
        // D s_1_3: write-var tablebase.1 <= s_1_2
        fn_state.tablebase._1 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_2_0: const #1u : u8
        let s_2_0: bool = true;
        // D s_2_1: write-var nextstate.5 <= s_2_0
        fn_state.nextstate._5 = s_2_0;
        // D s_2_2: read-var currentstate.8:struct
        let s_2_2: bool = fn_state.currentstate._8;
        // D s_2_3: write-var nextstate.8 <= s_2_2
        fn_state.nextstate._8 = s_2_2;
        // D s_2_4: read-var walkparams.3:struct
        let s_2_4: bool = fn_state.walkparams._3;
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // C s_2_6: const #1u : u8
        let s_2_6: bool = true;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // D s_2_8: cmp-eq s_2_5 s_2_7
        let s_2_8: bool = ((s_2_5) == (s_2_7));
        // N s_2_9: branch s_2_8 b29 b3
        if s_2_8 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_3_0: read-var currentstate.6:struct
        let s_3_0: i128 = fn_state.currentstate._6;
        // C s_3_1: const #1s : i
        let s_3_1: i128 = 1;
        // D s_3_2: add s_3_0 s_3_1
        let s_3_2: i128 = (s_3_0 + s_3_1);
        // D s_3_3: write-var nextstate.6 <= s_3_2
        fn_state.nextstate._6 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_4_0: read-var tablebase:struct
        let s_4_0: ProductTypeda0231e9dc169f81 = fn_state.tablebase;
        // D s_4_1: write-var nextstate.0 <= s_4_0
        fn_state.nextstate._0 = s_4_0;
        // D s_4_2: read-var currentstate.7:struct
        let s_4_2: ProductTypef170cab34335b70c = fn_state.currentstate._7;
        // D s_4_3: write-var nextstate.7 <= s_4_2
        fn_state.nextstate._7 = s_4_2;
        // D s_4_4: read-var walkparams.15:struct
        let s_4_4: bool = fn_state.walkparams._15;
        // D s_4_5: cast zx s_4_4 -> bv
        let s_4_5: Bits = Bits::new(s_4_4 as u128, 1u16);
        // C s_4_6: const #0u : u8
        let s_4_6: bool = false;
        // C s_4_7: cast zx s_4_6 -> bv
        let s_4_7: Bits = Bits::new(s_4_6 as u128, 1u16);
        // D s_4_8: cmp-eq s_4_5 s_4_7
        let s_4_8: bool = ((s_4_5) == (s_4_7));
        // N s_4_9: branch s_4_8 b28 b5
        if s_4_8 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#18185 <= s_5_0
        fn_state.gs_18185 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_6_0: read-var gs#18185:u8
        let s_6_0: bool = fn_state.gs_18185;
        // N s_6_1: branch s_6_0 b27 b7
        if s_6_0 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_7_0: read-var currentstate.9:struct
        let s_7_0: ProductTypebf05c51f33174538 = fn_state.currentstate._9;
        // D s_7_1: write-var nextstate.9 <= s_7_0
        fn_state.nextstate._9 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_8_0: read-var walkparams.3:struct
        let s_8_0: bool = fn_state.walkparams._3;
        // D s_8_1: cast zx s_8_0 -> bv
        let s_8_1: Bits = Bits::new(s_8_0 as u128, 1u16);
        // C s_8_2: const #1u : u8
        let s_8_2: bool = true;
        // C s_8_3: cast zx s_8_2 -> bv
        let s_8_3: Bits = Bits::new(s_8_2 as u128, 1u16);
        // D s_8_4: cmp-eq s_8_1 s_8_3
        let s_8_4: bool = ((s_8_1) == (s_8_3));
        // N s_8_5: branch s_8_4 b26 b9
        if s_8_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_9_0: read-var walkparams.27:struct
        let s_9_0: bool = fn_state.walkparams._27;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b25 b10
        if s_9_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var protectedbit <= s_10_0
        fn_state.protectedbit = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_12_0: read-var currentstate.10:struct
        let s_12_0: bool = fn_state.currentstate._10;
        // N s_12_1: branch s_12_0 b24 b13
        if s_12_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#18208 <= s_13_0
        fn_state.gs_18208 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_14_0: read-var gs#18208:u8
        let s_14_0: bool = fn_state.gs_18208;
        // N s_14_1: branch s_14_0 b23 b15
        if s_14_0 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#18209 <= s_15_0
        fn_state.gs_18209 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_16_0: read-var gs#18209:u8
        let s_16_0: bool = fn_state.gs_18209;
        // N s_16_1: branch s_16_0 b22 b17
        if s_16_0 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var nextstate.10 <= s_17_0
        fn_state.nextstate._10 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_18_0: read-var walkparams.3:struct
        let s_18_0: bool = fn_state.walkparams._3;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #1u : u8
        let s_18_2: bool = true;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b21 b19
        if s_18_4 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_19_0: const #0u : u8
        let s_19_0: bool = false;
        // D s_19_1: write-var nextstate.2 <= s_19_0
        fn_state.nextstate._2 = s_19_0;
        // N s_19_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_20_0: read-var nextstate:struct
        let s_20_0: ProductType96e7acababe246a1 = fn_state.nextstate;
        // N s_20_1: return s_20_0
        return s_20_0;
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_21_0: read-var descriptor:bv
        let s_21_0: Bits = fn_state.descriptor;
        // D s_21_1: size-of s_21_0
        let s_21_1: u16 = s_21_0.length();
        // D s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (i128::try_from(s_21_1).unwrap());
        // C s_21_3: const #112s : i
        let s_21_3: i128 = 112;
        // D s_21_4: cmp-lt s_21_3 s_21_2
        let s_21_4: bool = ((s_21_3) < (s_21_2));
        // N s_21_5: assert s_21_4
        let s_21_5: () = assert!(s_21_4);
        // C s_21_6: const #112s : i
        let s_21_6: i128 = 112;
        // D s_21_7: read-var descriptor:bv
        let s_21_7: Bits = fn_state.descriptor;
        // C s_21_8: const #1u : u64
        let s_21_8: u64 = 1;
        // D s_21_9: bit-extract s_21_7 s_21_6 s_21_8
        let s_21_9: Bits = (Bits::new(
            ((s_21_7) >> (s_21_6)).value(),
            u16::try_from(s_21_8).unwrap(),
        ));
        // D s_21_10: cast reint s_21_9 -> u8
        let s_21_10: bool = ((s_21_9.value()) != 0);
        // C s_21_11: const #0s : i
        let s_21_11: i128 = 0;
        // C s_21_12: const #0u : u64
        let s_21_12: u64 = 0;
        // D s_21_13: cast zx s_21_10 -> u64
        let s_21_13: u64 = (s_21_10 as u64);
        // C s_21_14: const #1u : u64
        let s_21_14: u64 = 1;
        // D s_21_15: and s_21_13 s_21_14
        let s_21_15: u64 = ((s_21_13) & (s_21_14));
        // D s_21_16: cmp-eq s_21_15 s_21_14
        let s_21_16: bool = ((s_21_15) == (s_21_14));
        // D s_21_17: lsl s_21_13 s_21_11
        let s_21_17: u64 = s_21_13 << s_21_11;
        // D s_21_18: or s_21_12 s_21_17
        let s_21_18: u64 = ((s_21_12) | (s_21_17));
        // D s_21_19: cmpl s_21_17
        let s_21_19: u64 = !s_21_17;
        // D s_21_20: and s_21_12 s_21_19
        let s_21_20: u64 = ((s_21_12) & (s_21_19));
        // D s_21_21: select s_21_16 s_21_18 s_21_20
        let s_21_21: u64 = if s_21_16 { s_21_18 } else { s_21_20 };
        // D s_21_22: cast trunc s_21_21 -> u8
        let s_21_22: bool = ((s_21_21) != 0);
        // D s_21_23: write-var nextstate.2 <= s_21_22
        fn_state.nextstate._2 = s_21_22;
        // N s_21_24: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_22_0: const #1u : u8
        let s_22_0: bool = true;
        // D s_22_1: write-var nextstate.10 <= s_22_0
        fn_state.nextstate._10 = s_22_0;
        // N s_22_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_23_0: read-var protectedbit:u8
        let s_23_0: bool = fn_state.protectedbit;
        // D s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 1u16);
        // C s_23_2: const #1u : u8
        let s_23_2: bool = true;
        // C s_23_3: cast zx s_23_2 -> bv
        let s_23_3: Bits = Bits::new(s_23_2 as u128, 1u16);
        // D s_23_4: cmp-eq s_23_1 s_23_3
        let s_23_4: bool = ((s_23_1) == (s_23_3));
        // D s_23_5: write-var gs#18209 <= s_23_4
        fn_state.gs_18209 = s_23_4;
        // N s_23_6: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_24_0: read-var s2fs1mro:u8
        let s_24_0: bool = fn_state.s2fs1mro;
        // D s_24_1: write-var gs#18208 <= s_24_0
        fn_state.gs_18208 = s_24_0;
        // N s_24_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_25_0: read-var descriptor:bv
        let s_25_0: Bits = fn_state.descriptor;
        // D s_25_1: size-of s_25_0
        let s_25_1: u16 = s_25_0.length();
        // D s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (i128::try_from(s_25_1).unwrap());
        // C s_25_3: const #52s : i
        let s_25_3: i128 = 52;
        // D s_25_4: cmp-lt s_25_3 s_25_2
        let s_25_4: bool = ((s_25_3) < (s_25_2));
        // N s_25_5: assert s_25_4
        let s_25_5: () = assert!(s_25_4);
        // C s_25_6: const #52s : i
        let s_25_6: i128 = 52;
        // D s_25_7: read-var descriptor:bv
        let s_25_7: Bits = fn_state.descriptor;
        // C s_25_8: const #1u : u64
        let s_25_8: u64 = 1;
        // D s_25_9: bit-extract s_25_7 s_25_6 s_25_8
        let s_25_9: Bits = (Bits::new(
            ((s_25_7) >> (s_25_6)).value(),
            u16::try_from(s_25_8).unwrap(),
        ));
        // D s_25_10: cast reint s_25_9 -> u8
        let s_25_10: bool = ((s_25_9.value()) != 0);
        // C s_25_11: const #0s : i
        let s_25_11: i128 = 0;
        // C s_25_12: const #0u : u64
        let s_25_12: u64 = 0;
        // D s_25_13: cast zx s_25_10 -> u64
        let s_25_13: u64 = (s_25_10 as u64);
        // C s_25_14: const #1u : u64
        let s_25_14: u64 = 1;
        // D s_25_15: and s_25_13 s_25_14
        let s_25_15: u64 = ((s_25_13) & (s_25_14));
        // D s_25_16: cmp-eq s_25_15 s_25_14
        let s_25_16: bool = ((s_25_15) == (s_25_14));
        // D s_25_17: lsl s_25_13 s_25_11
        let s_25_17: u64 = s_25_13 << s_25_11;
        // D s_25_18: or s_25_12 s_25_17
        let s_25_18: u64 = ((s_25_12) | (s_25_17));
        // D s_25_19: cmpl s_25_17
        let s_25_19: u64 = !s_25_17;
        // D s_25_20: and s_25_12 s_25_19
        let s_25_20: u64 = ((s_25_12) & (s_25_19));
        // D s_25_21: select s_25_16 s_25_18 s_25_20
        let s_25_21: u64 = if s_25_16 { s_25_18 } else { s_25_20 };
        // D s_25_22: cast trunc s_25_21 -> u8
        let s_25_22: bool = ((s_25_21) != 0);
        // D s_25_23: write-var protectedbit <= s_25_22
        fn_state.protectedbit = s_25_22;
        // N s_25_24: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_26_0: read-var descriptor:bv
        let s_26_0: Bits = fn_state.descriptor;
        // D s_26_1: size-of s_26_0
        let s_26_1: u16 = s_26_0.length();
        // D s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (i128::try_from(s_26_1).unwrap());
        // C s_26_3: const #114s : i
        let s_26_3: i128 = 114;
        // D s_26_4: cmp-lt s_26_3 s_26_2
        let s_26_4: bool = ((s_26_3) < (s_26_2));
        // N s_26_5: assert s_26_4
        let s_26_5: () = assert!(s_26_4);
        // C s_26_6: const #114s : i
        let s_26_6: i128 = 114;
        // D s_26_7: read-var descriptor:bv
        let s_26_7: Bits = fn_state.descriptor;
        // C s_26_8: const #1u : u64
        let s_26_8: u64 = 1;
        // D s_26_9: bit-extract s_26_7 s_26_6 s_26_8
        let s_26_9: Bits = (Bits::new(
            ((s_26_7) >> (s_26_6)).value(),
            u16::try_from(s_26_8).unwrap(),
        ));
        // D s_26_10: cast reint s_26_9 -> u8
        let s_26_10: bool = ((s_26_9.value()) != 0);
        // C s_26_11: const #0s : i
        let s_26_11: i128 = 0;
        // C s_26_12: const #0u : u64
        let s_26_12: u64 = 0;
        // D s_26_13: cast zx s_26_10 -> u64
        let s_26_13: u64 = (s_26_10 as u64);
        // C s_26_14: const #1u : u64
        let s_26_14: u64 = 1;
        // D s_26_15: and s_26_13 s_26_14
        let s_26_15: u64 = ((s_26_13) & (s_26_14));
        // D s_26_16: cmp-eq s_26_15 s_26_14
        let s_26_16: bool = ((s_26_15) == (s_26_14));
        // D s_26_17: lsl s_26_13 s_26_11
        let s_26_17: u64 = s_26_13 << s_26_11;
        // D s_26_18: or s_26_12 s_26_17
        let s_26_18: u64 = ((s_26_12) | (s_26_17));
        // D s_26_19: cmpl s_26_17
        let s_26_19: u64 = !s_26_17;
        // D s_26_20: and s_26_12 s_26_19
        let s_26_20: u64 = ((s_26_12) & (s_26_19));
        // D s_26_21: select s_26_16 s_26_18 s_26_20
        let s_26_21: u64 = if s_26_16 { s_26_18 } else { s_26_20 };
        // D s_26_22: cast trunc s_26_21 -> u8
        let s_26_22: bool = ((s_26_21) != 0);
        // D s_26_23: write-var protectedbit <= s_26_22
        fn_state.protectedbit = s_26_22;
        // N s_26_24: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_27_0: read-var currentstate.9:struct
        let s_27_0: ProductTypebf05c51f33174538 = fn_state.currentstate._9;
        // D s_27_1: read-var descriptor:bv
        let s_27_1: Bits = fn_state.descriptor;
        // D s_27_2: read-var regime:u32
        let s_27_2: u32 = fn_state.regime;
        // D s_27_3: read-var walkparams:struct
        let s_27_3: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_27_4: call AArch64_S1ApplyTablePerms(s_27_0, s_27_1, s_27_2, s_27_3)
        let s_27_4: ProductTypebf05c51f33174538 = AArch64_S1ApplyTablePerms(
            state,
            tracer,
            s_27_0,
            s_27_1,
            s_27_2,
            s_27_3,
        );
        // D s_27_5: write-var nextstate.9 <= s_27_4
        fn_state.nextstate._9 = s_27_4;
        // N s_27_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_28_0: read-var walkparams.24:struct
        let s_28_0: bool = fn_state.walkparams._24;
        // D s_28_1: cast zx s_28_0 -> bv
        let s_28_1: Bits = Bits::new(s_28_0 as u128, 1u16);
        // C s_28_2: const #0u : u8
        let s_28_2: bool = false;
        // C s_28_3: cast zx s_28_2 -> bv
        let s_28_3: Bits = Bits::new(s_28_2 as u128, 1u16);
        // D s_28_4: cmp-eq s_28_1 s_28_3
        let s_28_4: bool = ((s_28_1) == (s_28_3));
        // D s_28_5: write-var gs#18185 <= s_28_4
        fn_state.gs_18185 = s_28_4;
        // N s_28_6: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_29_0: read-var descriptor:bv
        let s_29_0: Bits = fn_state.descriptor;
        // D s_29_1: size-of s_29_0
        let s_29_1: u16 = s_29_0.length();
        // D s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (i128::try_from(s_29_1).unwrap());
        // C s_29_3: const #110s : i
        let s_29_3: i128 = 110;
        // D s_29_4: cmp-lt s_29_3 s_29_2
        let s_29_4: bool = ((s_29_3) < (s_29_2));
        // N s_29_5: assert s_29_4
        let s_29_5: () = assert!(s_29_4);
        // C s_29_6: const #109s : i
        let s_29_6: i128 = 109;
        // D s_29_7: read-var descriptor:bv
        let s_29_7: Bits = fn_state.descriptor;
        // C s_29_8: const #1s : i64
        let s_29_8: i64 = 1;
        // C s_29_9: cast zx s_29_8 -> i
        let s_29_9: i128 = (i128::try_from(s_29_8).unwrap());
        // C s_29_10: const #1s : i
        let s_29_10: i128 = 1;
        // C s_29_11: add s_29_10 s_29_9
        let s_29_11: i128 = (s_29_10 + s_29_9);
        // D s_29_12: bit-extract s_29_7 s_29_6 s_29_11
        let s_29_12: Bits = (Bits::new(
            ((s_29_7) >> (s_29_6)).value(),
            u16::try_from(s_29_11).unwrap(),
        ));
        // D s_29_13: cast reint s_29_12 -> u8
        let s_29_13: u8 = (s_29_12.value() as u8);
        // D s_29_14: read-var currentstate.6:struct
        let s_29_14: i128 = fn_state.currentstate._6;
        // D s_29_15: cast zx s_29_13 -> bv
        let s_29_15: Bits = Bits::new(s_29_13 as u128, 2u16);
        // D s_29_16: cast zx s_29_15 -> i
        let s_29_16: i128 = (s_29_15.value() as i128);
        // D s_29_17: cast reint s_29_16 -> i64
        let s_29_17: i64 = (s_29_16 as i64);
        // D s_29_18: cast zx s_29_17 -> i
        let s_29_18: i128 = (i128::try_from(s_29_17).unwrap());
        // D s_29_19: add s_29_14 s_29_18
        let s_29_19: i128 = (s_29_14 + s_29_18);
        // C s_29_20: const #1s : i
        let s_29_20: i128 = 1;
        // D s_29_21: add s_29_19 s_29_20
        let s_29_21: i128 = (s_29_19 + s_29_20);
        // D s_29_22: write-var nextstate.6 <= s_29_21
        fn_state.nextstate._6 = s_29_21;
        // N s_29_23: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_30_0: read-var walkparams.3:struct
        let s_30_0: bool = fn_state.walkparams._3;
        // D s_30_1: cast zx s_30_0 -> bv
        let s_30_1: Bits = Bits::new(s_30_0 as u128, 1u16);
        // C s_30_2: const #1u : u8
        let s_30_2: bool = true;
        // C s_30_3: cast zx s_30_2 -> bv
        let s_30_3: Bits = Bits::new(s_30_2 as u128, 1u16);
        // D s_30_4: cmp-eq s_30_1 s_30_3
        let s_30_4: bool = ((s_30_1) == (s_30_3));
        // N s_30_5: branch s_30_4 b36 b31
        if s_30_4 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_31_0: read-var descriptor:bv
        let s_31_0: Bits = fn_state.descriptor;
        // D s_31_1: size-of s_31_0
        let s_31_1: u16 = s_31_0.length();
        // D s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (i128::try_from(s_31_1).unwrap());
        // C s_31_3: const #63s : i
        let s_31_3: i128 = 63;
        // D s_31_4: cmp-lt s_31_3 s_31_2
        let s_31_4: bool = ((s_31_3) < (s_31_2));
        // N s_31_5: assert s_31_4
        let s_31_5: () = assert!(s_31_4);
        // C s_31_6: const #63s : i
        let s_31_6: i128 = 63;
        // D s_31_7: read-var descriptor:bv
        let s_31_7: Bits = fn_state.descriptor;
        // C s_31_8: const #1u : u64
        let s_31_8: u64 = 1;
        // D s_31_9: bit-extract s_31_7 s_31_6 s_31_8
        let s_31_9: Bits = (Bits::new(
            ((s_31_7) >> (s_31_6)).value(),
            u16::try_from(s_31_8).unwrap(),
        ));
        // D s_31_10: cast reint s_31_9 -> u8
        let s_31_10: bool = ((s_31_9.value()) != 0);
        // C s_31_11: const #0s : i
        let s_31_11: i128 = 0;
        // C s_31_12: const #0u : u64
        let s_31_12: u64 = 0;
        // D s_31_13: cast zx s_31_10 -> u64
        let s_31_13: u64 = (s_31_10 as u64);
        // C s_31_14: const #1u : u64
        let s_31_14: u64 = 1;
        // D s_31_15: and s_31_13 s_31_14
        let s_31_15: u64 = ((s_31_13) & (s_31_14));
        // D s_31_16: cmp-eq s_31_15 s_31_14
        let s_31_16: bool = ((s_31_15) == (s_31_14));
        // D s_31_17: lsl s_31_13 s_31_11
        let s_31_17: u64 = s_31_13 << s_31_11;
        // D s_31_18: or s_31_12 s_31_17
        let s_31_18: u64 = ((s_31_12) | (s_31_17));
        // D s_31_19: cmpl s_31_17
        let s_31_19: u64 = !s_31_17;
        // D s_31_20: and s_31_12 s_31_19
        let s_31_20: u64 = ((s_31_12) & (s_31_19));
        // D s_31_21: select s_31_16 s_31_18 s_31_20
        let s_31_21: u64 = if s_31_16 { s_31_18 } else { s_31_20 };
        // D s_31_22: cast trunc s_31_21 -> u8
        let s_31_22: bool = ((s_31_21) != 0);
        // D s_31_23: write-var nstable <= s_31_22
        fn_state.nstable = s_31_22;
        // N s_31_24: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_32_0: read-var nstable:u8
        let s_32_0: bool = fn_state.nstable;
        // D s_32_1: cast zx s_32_0 -> bv
        let s_32_1: Bits = Bits::new(s_32_0 as u128, 1u16);
        // C s_32_2: const #0u : u8
        let s_32_2: bool = false;
        // C s_32_3: cast zx s_32_2 -> bv
        let s_32_3: Bits = Bits::new(s_32_2 as u128, 1u16);
        // D s_32_4: cmp-eq s_32_1 s_32_3
        let s_32_4: bool = ((s_32_1) == (s_32_3));
        // N s_32_5: branch s_32_4 b35 b33
        if s_32_4 {
            return block_35(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_33_0: const #0u : u32
        let s_33_0: u32 = 0;
        // D s_33_1: write-var tablebase.1 <= s_33_0
        fn_state.tablebase._1 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_34_0: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_35_0: const #1u : u32
        let s_35_0: u32 = 1;
        // D s_35_1: write-var tablebase.1 <= s_35_0
        fn_state.tablebase._1 = s_35_0;
        // N s_35_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_36_0: read-var descriptor:bv
        let s_36_0: Bits = fn_state.descriptor;
        // D s_36_1: size-of s_36_0
        let s_36_1: u16 = s_36_0.length();
        // D s_36_2: cast zx s_36_1 -> i
        let s_36_2: i128 = (i128::try_from(s_36_1).unwrap());
        // C s_36_3: const #127s : i
        let s_36_3: i128 = 127;
        // D s_36_4: cmp-lt s_36_3 s_36_2
        let s_36_4: bool = ((s_36_3) < (s_36_2));
        // N s_36_5: assert s_36_4
        let s_36_5: () = assert!(s_36_4);
        // C s_36_6: const #127s : i
        let s_36_6: i128 = 127;
        // D s_36_7: read-var descriptor:bv
        let s_36_7: Bits = fn_state.descriptor;
        // C s_36_8: const #1u : u64
        let s_36_8: u64 = 1;
        // D s_36_9: bit-extract s_36_7 s_36_6 s_36_8
        let s_36_9: Bits = (Bits::new(
            ((s_36_7) >> (s_36_6)).value(),
            u16::try_from(s_36_8).unwrap(),
        ));
        // D s_36_10: cast reint s_36_9 -> u8
        let s_36_10: bool = ((s_36_9.value()) != 0);
        // C s_36_11: const #0s : i
        let s_36_11: i128 = 0;
        // C s_36_12: const #0u : u64
        let s_36_12: u64 = 0;
        // D s_36_13: cast zx s_36_10 -> u64
        let s_36_13: u64 = (s_36_10 as u64);
        // C s_36_14: const #1u : u64
        let s_36_14: u64 = 1;
        // D s_36_15: and s_36_13 s_36_14
        let s_36_15: u64 = ((s_36_13) & (s_36_14));
        // D s_36_16: cmp-eq s_36_15 s_36_14
        let s_36_16: bool = ((s_36_15) == (s_36_14));
        // D s_36_17: lsl s_36_13 s_36_11
        let s_36_17: u64 = s_36_13 << s_36_11;
        // D s_36_18: or s_36_12 s_36_17
        let s_36_18: u64 = ((s_36_12) | (s_36_17));
        // D s_36_19: cmpl s_36_17
        let s_36_19: u64 = !s_36_17;
        // D s_36_20: and s_36_12 s_36_19
        let s_36_20: u64 = ((s_36_12) & (s_36_19));
        // D s_36_21: select s_36_16 s_36_18 s_36_20
        let s_36_21: u64 = if s_36_16 { s_36_18 } else { s_36_20 };
        // D s_36_22: cast trunc s_36_21 -> u8
        let s_36_22: bool = ((s_36_21) != 0);
        // D s_36_23: write-var nstable <= s_36_22
        fn_state.nstable = s_36_22;
        // N s_36_24: jump b32
        return block_32(state, tracer, fn_state);
    }
}
