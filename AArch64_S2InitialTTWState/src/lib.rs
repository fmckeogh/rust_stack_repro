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
use VTTBR_EL2_read::*;
use AArch64_S2TTBaseAddress::*;
use WalkMemAttrs::*;
use AArch64_S2StartLevel::*;
use common::*;
pub fn AArch64_S2InitialTTWState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    ss: u32,
    walkparams: ProductTypeb05ce25a107f0c5e,
) -> ProductType96e7acababe246a1 {
    #[derive(Default)]
    struct FunctionState {
        tablebase: ProductTypeda0231e9dc169f81,
        ga_14318: ProductType782ac6922b48c20d,
        ttbr: u128,
        walkstate: ProductType96e7acababe246a1,
        ss: u32,
        walkparams: ProductTypeb05ce25a107f0c5e,
    }
    let fn_state = FunctionState {
        ss,
        walkparams,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call VTTBR_EL2_read(s_0_0)
        let s_0_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_0_0);
        // D s_0_2: write-var ga#14318 <= s_0_1
        fn_state.ga_14318 = s_0_1;
        // D s_0_3: read-var ga#14318.0:struct
        let s_0_3: u128 = fn_state.ga_14318._0;
        // C s_0_4: const #128s : i
        let s_0_4: i128 = 128;
        // D s_0_5: cast zx s_0_3 -> bv
        let s_0_5: Bits = Bits::new(s_0_3 as u128, 128u16);
        // D s_0_6: bits-cast zx s_0_5 -> bv length s_0_4
        let s_0_6: Bits = s_0_5.zero_extend(s_0_4);
        // D s_0_7: cast reint s_0_6 -> u128
        let s_0_7: u128 = (s_0_6.value() as u128);
        // D s_0_8: write-var ttbr <= s_0_7
        fn_state.ttbr = s_0_7;
        // C s_0_9: const #0u : u32
        let s_0_9: u32 = 0;
        // D s_0_10: read-var ss:u32
        let s_0_10: u32 = fn_state.ss;
        // D s_0_11: cmp-eq s_0_9 s_0_10
        let s_0_11: bool = ((s_0_9) == (s_0_10));
        // D s_0_12: not s_0_11
        let s_0_12: bool = !s_0_11;
        // N s_0_13: branch s_0_12 b3 b1
        if s_0_12 {
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
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
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
        // D s_2_0: read-var tablebase.1:struct
        let s_2_0: u32 = fn_state.tablebase._1;
        // D s_2_1: read-var ttbr:u128
        let s_2_1: u128 = fn_state.ttbr;
        // D s_2_2: cast zx s_2_1 -> bv
        let s_2_2: Bits = Bits::new(s_2_1 as u128, 128u16);
        // D s_2_3: read-var walkparams:struct
        let s_2_3: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_2_4: call AArch64_S2TTBaseAddress(s_2_3, s_2_0, s_2_2)
        let s_2_4: u64 = AArch64_S2TTBaseAddress(state, tracer, s_2_3, s_2_0, s_2_2);
        // D s_2_5: write-var tablebase.0 <= s_2_4
        fn_state.tablebase._0 = s_2_4;
        // D s_2_6: read-var tablebase:struct
        let s_2_6: ProductTypeda0231e9dc169f81 = fn_state.tablebase;
        // D s_2_7: write-var walkstate.0 <= s_2_6
        fn_state.walkstate._0 = s_2_6;
        // D s_2_8: read-var walkparams:struct
        let s_2_8: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_2_9: call AArch64_S2StartLevel(s_2_8)
        let s_2_9: i128 = AArch64_S2StartLevel(state, tracer, s_2_8);
        // D s_2_10: write-var walkstate.6 <= s_2_9
        fn_state.walkstate._6 = s_2_9;
        // C s_2_11: const #1u : u8
        let s_2_11: bool = true;
        // D s_2_12: write-var walkstate.5 <= s_2_11
        fn_state.walkstate._5 = s_2_11;
        // D s_2_13: read-var walkparams.20:struct
        let s_2_13: u8 = fn_state.walkparams._20;
        // D s_2_14: read-var walkparams.10:struct
        let s_2_14: u8 = fn_state.walkparams._10;
        // D s_2_15: read-var walkparams.13:struct
        let s_2_15: u8 = fn_state.walkparams._13;
        // D s_2_16: call WalkMemAttrs(s_2_13, s_2_14, s_2_15)
        let s_2_16: ProductTypef170cab34335b70c = WalkMemAttrs(
            state,
            tracer,
            s_2_13,
            s_2_14,
            s_2_15,
        );
        // D s_2_17: write-var walkstate.7 <= s_2_16
        fn_state.walkstate._7 = s_2_16;
        // D s_2_18: read-var walkstate:struct
        let s_2_18: ProductType96e7acababe246a1 = fn_state.walkstate;
        // N s_2_19: return s_2_18
        return s_2_18;
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_3_0: const #2u : u32
        let s_3_0: u32 = 2;
        // D s_3_1: read-var ss:u32
        let s_3_1: u32 = fn_state.ss;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: not s_3_2
        let s_3_3: bool = !s_3_2;
        // N s_3_4: branch s_3_3 b5 b4
        if s_3_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_4_0: const #3u : u32
        let s_4_0: u32 = 3;
        // D s_4_1: write-var tablebase.1 <= s_4_0
        fn_state.tablebase._1 = s_4_0;
        // N s_4_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_5_0: jump b2
        return block_2(state, tracer, fn_state);
    }
}
