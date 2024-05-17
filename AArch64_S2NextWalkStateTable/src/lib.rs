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
use common::*;
pub fn AArch64_S2NextWalkStateTable<T: Tracer>(
    state: &mut State,
    tracer: &T,
    currentstate: ProductType96e7acababe246a1,
    walkparams: ProductTypeb05ce25a107f0c5e,
    descriptor: Bits,
) -> ProductType96e7acababe246a1 {
    #[derive(Default)]
    struct FunctionState {
        tablebase: ProductTypeda0231e9dc169f81,
        nextstate: ProductType96e7acababe246a1,
        ga_14444: ProductTypeda0231e9dc169f81,
        currentstate: ProductType96e7acababe246a1,
        walkparams: ProductTypeb05ce25a107f0c5e,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        currentstate,
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
        // D s_0_0: read-var walkparams.2:struct
        let s_0_0: bool = fn_state.walkparams._2;
        // D s_0_1: read-var walkparams.3:struct
        let s_0_1: bool = fn_state.walkparams._3;
        // D s_0_2: read-var walkparams.26:struct
        let s_0_2: u32 = fn_state.walkparams._26;
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
        // D s_0_7: write-var ga#14444 <= s_0_6
        fn_state.ga_14444 = s_0_6;
        // D s_0_8: read-var ga#14444.1:struct
        let s_0_8: u32 = fn_state.ga_14444._1;
        // D s_0_9: write-var tablebase.1 <= s_0_8
        fn_state.tablebase._1 = s_0_8;
        // C s_0_10: const #1u : u8
        let s_0_10: bool = true;
        // D s_0_11: write-var nextstate.5 <= s_0_10
        fn_state.nextstate._5 = s_0_10;
        // D s_0_12: read-var walkparams.2:struct
        let s_0_12: bool = fn_state.walkparams._2;
        // D s_0_13: cast zx s_0_12 -> bv
        let s_0_13: Bits = Bits::new(s_0_12 as u128, 1u16);
        // C s_0_14: const #1u : u8
        let s_0_14: bool = true;
        // C s_0_15: cast zx s_0_14 -> bv
        let s_0_15: Bits = Bits::new(s_0_14 as u128, 1u16);
        // D s_0_16: cmp-eq s_0_13 s_0_15
        let s_0_16: bool = ((s_0_13) == (s_0_15));
        // N s_0_17: branch s_0_16 b3 b1
        if s_0_16 {
            return block_3(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_1_0: read-var currentstate.6:struct
        let s_1_0: i128 = fn_state.currentstate._6;
        // C s_1_1: const #1s : i
        let s_1_1: i128 = 1;
        // D s_1_2: add s_1_0 s_1_1
        let s_1_2: i128 = (s_1_0 + s_1_1);
        // D s_1_3: write-var nextstate.6 <= s_1_2
        fn_state.nextstate._6 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_2_0: read-var tablebase:struct
        let s_2_0: ProductTypeda0231e9dc169f81 = fn_state.tablebase;
        // D s_2_1: write-var nextstate.0 <= s_2_0
        fn_state.nextstate._0 = s_2_0;
        // D s_2_2: read-var currentstate.7:struct
        let s_2_2: ProductTypef170cab34335b70c = fn_state.currentstate._7;
        // D s_2_3: write-var nextstate.7 <= s_2_2
        fn_state.nextstate._7 = s_2_2;
        // D s_2_4: read-var nextstate:struct
        let s_2_4: ProductType96e7acababe246a1 = fn_state.nextstate;
        // N s_2_5: return s_2_4
        return s_2_4;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_3_0: read-var descriptor:bv
        let s_3_0: Bits = fn_state.descriptor;
        // D s_3_1: size-of s_3_0
        let s_3_1: u16 = s_3_0.length();
        // D s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (i128::try_from(s_3_1).unwrap());
        // C s_3_3: const #110s : i
        let s_3_3: i128 = 110;
        // D s_3_4: cmp-lt s_3_3 s_3_2
        let s_3_4: bool = ((s_3_3) < (s_3_2));
        // N s_3_5: assert s_3_4
        let s_3_5: () = assert!(s_3_4);
        // C s_3_6: const #109s : i
        let s_3_6: i128 = 109;
        // D s_3_7: read-var descriptor:bv
        let s_3_7: Bits = fn_state.descriptor;
        // C s_3_8: const #1s : i64
        let s_3_8: i64 = 1;
        // C s_3_9: cast zx s_3_8 -> i
        let s_3_9: i128 = (i128::try_from(s_3_8).unwrap());
        // C s_3_10: const #1s : i
        let s_3_10: i128 = 1;
        // C s_3_11: add s_3_10 s_3_9
        let s_3_11: i128 = (s_3_10 + s_3_9);
        // D s_3_12: bit-extract s_3_7 s_3_6 s_3_11
        let s_3_12: Bits = (Bits::new(
            ((s_3_7) >> (s_3_6)).value(),
            u16::try_from(s_3_11).unwrap(),
        ));
        // D s_3_13: cast reint s_3_12 -> u8
        let s_3_13: u8 = (s_3_12.value() as u8);
        // D s_3_14: read-var currentstate.6:struct
        let s_3_14: i128 = fn_state.currentstate._6;
        // D s_3_15: cast zx s_3_13 -> bv
        let s_3_15: Bits = Bits::new(s_3_13 as u128, 2u16);
        // D s_3_16: cast zx s_3_15 -> i
        let s_3_16: i128 = (s_3_15.value() as i128);
        // D s_3_17: cast reint s_3_16 -> i64
        let s_3_17: i64 = (s_3_16 as i64);
        // D s_3_18: cast zx s_3_17 -> i
        let s_3_18: i128 = (i128::try_from(s_3_17).unwrap());
        // D s_3_19: add s_3_14 s_3_18
        let s_3_19: i128 = (s_3_14 + s_3_18);
        // C s_3_20: const #1s : i
        let s_3_20: i128 = 1;
        // D s_3_21: add s_3_19 s_3_20
        let s_3_21: i128 = (s_3_19 + s_3_20);
        // D s_3_22: write-var nextstate.6 <= s_3_21
        fn_state.nextstate._6 = s_3_21;
        // N s_3_23: jump b2
        return block_2(state, tracer, fn_state);
    }
}
