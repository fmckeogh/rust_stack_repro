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
use AArch64_S1TTBR::*;
use WalkMemAttrs::*;
use AArch64_S1StartLevel::*;
use AArch64_S1TTBaseAddress::*;
use HasUnprivileged::*;
use common::*;
pub fn AArch64_S1InitialTTWState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeef284266e139aee2,
    va: u64,
    regime: u32,
    ss: u32,
) -> ProductType96e7acababe246a1 {
    #[derive(Default)]
    struct FunctionState {
        tablebase: ProductTypeda0231e9dc169f81,
        gs_17775: bool,
        ttbr: u128,
        walkstate: ProductType96e7acababe246a1,
        permissions: ProductTypebf05c51f33174538,
        walkparams: ProductTypeef284266e139aee2,
        va: u64,
        regime: u32,
        ss: u32,
    }
    let fn_state = FunctionState {
        walkparams,
        va,
        regime,
        ss,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_0_0: read-var regime:u32
        let s_0_0: u32 = fn_state.regime;
        // D s_0_1: read-var va:u64
        let s_0_1: u64 = fn_state.va;
        // D s_0_2: call AArch64_S1TTBR(s_0_0, s_0_1)
        let s_0_2: u128 = AArch64_S1TTBR(state, tracer, s_0_0, s_0_1);
        // D s_0_3: write-var ttbr <= s_0_2
        fn_state.ttbr = s_0_2;
        // C s_0_4: const #3u : u32
        let s_0_4: u32 = 3;
        // D s_0_5: read-var ss:u32
        let s_0_5: u32 = fn_state.ss;
        // D s_0_6: cmp-eq s_0_4 s_0_5
        let s_0_6: bool = ((s_0_4) == (s_0_5));
        // D s_0_7: not s_0_6
        let s_0_7: bool = !s_0_6;
        // N s_0_8: branch s_0_7 b15 b1
        if s_0_7 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_1_0: const #1u : u32
        let s_1_0: u32 = 1;
        // D s_1_1: write-var tablebase.1 <= s_1_0
        fn_state.tablebase._1 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_2_0: read-var ttbr:u128
        let s_2_0: u128 = fn_state.ttbr;
        // D s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 128u16);
        // D s_2_2: read-var walkparams:struct
        let s_2_2: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_2_3: read-var regime:u32
        let s_2_3: u32 = fn_state.regime;
        // D s_2_4: call AArch64_S1TTBaseAddress(s_2_2, s_2_3, s_2_1)
        let s_2_4: u64 = AArch64_S1TTBaseAddress(state, tracer, s_2_2, s_2_3, s_2_1);
        // D s_2_5: write-var tablebase.0 <= s_2_4
        fn_state.tablebase._0 = s_2_4;
        // C s_2_6: const #0u : u8
        let s_2_6: u8 = 0;
        // D s_2_7: write-var permissions.1 <= s_2_6
        fn_state.permissions._1 = s_2_6;
        // D s_2_8: read-var regime:u32
        let s_2_8: u32 = fn_state.regime;
        // D s_2_9: call HasUnprivileged(s_2_8)
        let s_2_9: bool = HasUnprivileged(state, tracer, s_2_8);
        // N s_2_10: branch s_2_9 b14 b3
        if s_2_9 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var permissions.18 <= s_3_0
        fn_state.permissions._18 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_4_0: read-var tablebase:struct
        let s_4_0: ProductTypeda0231e9dc169f81 = fn_state.tablebase;
        // D s_4_1: write-var walkstate.0 <= s_4_0
        fn_state.walkstate._0 = s_4_0;
        // D s_4_2: read-var walkparams:struct
        let s_4_2: ProductTypeef284266e139aee2 = fn_state.walkparams;
        // D s_4_3: call AArch64_S1StartLevel(s_4_2)
        let s_4_3: i128 = AArch64_S1StartLevel(state, tracer, s_4_2);
        // D s_4_4: write-var walkstate.6 <= s_4_3
        fn_state.walkstate._6 = s_4_3;
        // C s_4_5: const #1u : u8
        let s_4_5: bool = true;
        // D s_4_6: write-var walkstate.5 <= s_4_5
        fn_state.walkstate._5 = s_4_5;
        // D s_4_7: read-var regime:u32
        let s_4_7: u32 = fn_state.regime;
        // D s_4_8: call HasUnprivileged(s_4_7)
        let s_4_8: bool = HasUnprivileged(state, tracer, s_4_7);
        // N s_4_9: branch s_4_8 b13 b5
        if s_4_8 {
            return block_13(state, tracer, fn_state);
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
        // D s_5_1: write-var walkstate.8 <= s_5_0
        fn_state.walkstate._8 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_6_0: read-var walkparams.29:struct
        let s_6_0: u8 = fn_state.walkparams._29;
        // D s_6_1: read-var walkparams.16:struct
        let s_6_1: u8 = fn_state.walkparams._16;
        // D s_6_2: read-var walkparams.23:struct
        let s_6_2: u8 = fn_state.walkparams._23;
        // D s_6_3: call WalkMemAttrs(s_6_0, s_6_1, s_6_2)
        let s_6_3: ProductTypef170cab34335b70c = WalkMemAttrs(
            state,
            tracer,
            s_6_0,
            s_6_1,
            s_6_2,
        );
        // D s_6_4: write-var walkstate.7 <= s_6_3
        fn_state.walkstate._7 = s_6_3;
        // D s_6_5: read-var permissions:struct
        let s_6_5: ProductTypebf05c51f33174538 = fn_state.permissions;
        // D s_6_6: write-var walkstate.9 <= s_6_5
        fn_state.walkstate._9 = s_6_5;
        // D s_6_7: read-var walkparams.3:struct
        let s_6_7: bool = fn_state.walkparams._3;
        // D s_6_8: cast zx s_6_7 -> bv
        let s_6_8: Bits = Bits::new(s_6_7 as u128, 1u16);
        // C s_6_9: const #1u : u8
        let s_6_9: bool = true;
        // C s_6_10: cast zx s_6_9 -> bv
        let s_6_10: Bits = Bits::new(s_6_9 as u128, 1u16);
        // D s_6_11: cmp-eq s_6_8 s_6_10
        let s_6_11: bool = ((s_6_8) == (s_6_10));
        // N s_6_12: branch s_6_11 b12 b7
        if s_6_11 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_7_0: read-var walkparams.27:struct
        let s_7_0: bool = fn_state.walkparams._27;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #1u : u8
        let s_7_2: bool = true;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // D s_7_5: write-var gs#17775 <= s_7_4
        fn_state.gs_17775 = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_8_0: read-var gs#17775:u8
        let s_8_0: bool = fn_state.gs_17775;
        // N s_8_1: branch s_8_0 b11 b9
        if s_8_0 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var walkstate.10 <= s_9_0
        fn_state.walkstate._10 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_10_0: read-var walkparams.6:struct
        let s_10_0: bool = fn_state.walkparams._6;
        // D s_10_1: write-var walkstate.2 <= s_10_0
        fn_state.walkstate._2 = s_10_0;
        // D s_10_2: read-var walkstate:struct
        let s_10_2: ProductType96e7acababe246a1 = fn_state.walkstate;
        // N s_10_3: return s_10_2
        return s_10_2;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var walkstate.10 <= s_11_0
        fn_state.walkstate._10 = s_11_0;
        // N s_11_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var gs#17775 <= s_12_0
        fn_state.gs_17775 = s_12_0;
        // N s_12_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var walkstate.8 <= s_13_0
        fn_state.walkstate._8 = s_13_0;
        // N s_13_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var permissions.16 <= s_14_0
        fn_state.permissions._16 = s_14_0;
        // C s_14_2: const #0u : u8
        let s_14_2: bool = false;
        // D s_14_3: write-var permissions.6 <= s_14_2
        fn_state.permissions._6 = s_14_2;
        // N s_14_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_15_0: const #0u : u32
        let s_15_0: u32 = 0;
        // D s_15_1: read-var ss:u32
        let s_15_1: u32 = fn_state.ss;
        // D s_15_2: cmp-eq s_15_0 s_15_1
        let s_15_2: bool = ((s_15_0) == (s_15_1));
        // D s_15_3: not s_15_2
        let s_15_3: bool = !s_15_2;
        // N s_15_4: branch s_15_3 b17 b16
        if s_15_3 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_16_0: const #0u : u32
        let s_16_0: u32 = 0;
        // D s_16_1: write-var tablebase.1 <= s_16_0
        fn_state.tablebase._1 = s_16_0;
        // N s_16_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_17_0: const #1u : u32
        let s_17_0: u32 = 1;
        // D s_17_1: read-var ss:u32
        let s_17_1: u32 = fn_state.ss;
        // D s_17_2: cmp-eq s_17_0 s_17_1
        let s_17_2: bool = ((s_17_0) == (s_17_1));
        // D s_17_3: not s_17_2
        let s_17_3: bool = !s_17_2;
        // N s_17_4: branch s_17_3 b19 b18
        if s_17_3 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_18_0: const #2u : u32
        let s_18_0: u32 = 2;
        // D s_18_1: write-var tablebase.1 <= s_18_0
        fn_state.tablebase._1 = s_18_0;
        // N s_18_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_19_0: const #2u : u32
        let s_19_0: u32 = 2;
        // D s_19_1: read-var ss:u32
        let s_19_1: u32 = fn_state.ss;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // D s_19_3: not s_19_2
        let s_19_3: bool = !s_19_2;
        // N s_19_4: branch s_19_3 b21 b20
        if s_19_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_20_0: const #3u : u32
        let s_20_0: u32 = 3;
        // D s_20_1: write-var tablebase.1 <= s_20_0
        fn_state.tablebase._1 = s_20_0;
        // N s_20_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_21_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
