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
pub fn AArch64_SS2InitialTTWState<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
    ipaspace: u32,
) -> ProductType96e7acababe246a1 {
    #[derive(Default)]
    struct FunctionState {
        tablebase: ProductTypeda0231e9dc169f81,
        ttbr: u128,
        walkstate: ProductType96e7acababe246a1,
        ga_14483: ProductType782ac6922b48c20d,
        walkparams: ProductTypeb05ce25a107f0c5e,
        ipaspace: u32,
    }
    let fn_state = FunctionState {
        walkparams,
        ipaspace,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_0_0: read-var ipaspace:u32
        let s_0_0: u32 = fn_state.ipaspace;
        // C s_0_1: const #1u : u32
        let s_0_1: u32 = 1;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b10 b1
        if s_0_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call VTTBR_EL2_read(s_1_0)
        let s_1_1: ProductType782ac6922b48c20d = VTTBR_EL2_read(state, tracer, s_1_0);
        // D s_1_2: write-var ga#14483 <= s_1_1
        fn_state.ga_14483 = s_1_1;
        // D s_1_3: read-var ga#14483.0:struct
        let s_1_3: u128 = fn_state.ga_14483._0;
        // C s_1_4: const #128s : i
        let s_1_4: i128 = 128;
        // D s_1_5: cast zx s_1_3 -> bv
        let s_1_5: Bits = Bits::new(s_1_3 as u128, 128u16);
        // D s_1_6: bits-cast zx s_1_5 -> bv length s_1_4
        let s_1_6: Bits = s_1_5.zero_extend(s_1_4);
        // D s_1_7: cast reint s_1_6 -> u128
        let s_1_7: u128 = (s_1_6.value() as u128);
        // D s_1_8: write-var ttbr <= s_1_7
        fn_state.ttbr = s_1_7;
        // N s_1_9: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_2_0: read-var ipaspace:u32
        let s_2_0: u32 = fn_state.ipaspace;
        // C s_2_1: const #1u : u32
        let s_2_1: u32 = 1;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b7 b3
        if s_2_2 {
            return block_7(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_3_0: read-var walkparams.12:struct
        let s_3_0: bool = fn_state.walkparams._12;
        // D s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 1u16);
        // C s_3_2: const #0u : u8
        let s_3_2: bool = false;
        // C s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 1u16);
        // D s_3_4: cmp-eq s_3_1 s_3_3
        let s_3_4: bool = ((s_3_1) == (s_3_3));
        // N s_3_5: branch s_3_4 b6 b4
        if s_3_4 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_4_0: const #0u : u32
        let s_4_0: u32 = 0;
        // D s_4_1: write-var tablebase.1 <= s_4_0
        fn_state.tablebase._1 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_5_0: read-var tablebase.1:struct
        let s_5_0: u32 = fn_state.tablebase._1;
        // D s_5_1: read-var ttbr:u128
        let s_5_1: u128 = fn_state.ttbr;
        // D s_5_2: cast zx s_5_1 -> bv
        let s_5_2: Bits = Bits::new(s_5_1 as u128, 128u16);
        // D s_5_3: read-var walkparams:struct
        let s_5_3: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_5_4: call AArch64_S2TTBaseAddress(s_5_3, s_5_0, s_5_2)
        let s_5_4: u64 = AArch64_S2TTBaseAddress(state, tracer, s_5_3, s_5_0, s_5_2);
        // D s_5_5: write-var tablebase.0 <= s_5_4
        fn_state.tablebase._0 = s_5_4;
        // D s_5_6: read-var tablebase:struct
        let s_5_6: ProductTypeda0231e9dc169f81 = fn_state.tablebase;
        // D s_5_7: write-var walkstate.0 <= s_5_6
        fn_state.walkstate._0 = s_5_6;
        // D s_5_8: read-var walkparams:struct
        let s_5_8: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_5_9: call AArch64_S2StartLevel(s_5_8)
        let s_5_9: i128 = AArch64_S2StartLevel(state, tracer, s_5_8);
        // D s_5_10: write-var walkstate.6 <= s_5_9
        fn_state.walkstate._6 = s_5_9;
        // C s_5_11: const #1u : u8
        let s_5_11: bool = true;
        // D s_5_12: write-var walkstate.5 <= s_5_11
        fn_state.walkstate._5 = s_5_11;
        // D s_5_13: read-var walkparams.20:struct
        let s_5_13: u8 = fn_state.walkparams._20;
        // D s_5_14: read-var walkparams.10:struct
        let s_5_14: u8 = fn_state.walkparams._10;
        // D s_5_15: read-var walkparams.13:struct
        let s_5_15: u8 = fn_state.walkparams._13;
        // D s_5_16: call WalkMemAttrs(s_5_13, s_5_14, s_5_15)
        let s_5_16: ProductTypef170cab34335b70c = WalkMemAttrs(
            state,
            tracer,
            s_5_13,
            s_5_14,
            s_5_15,
        );
        // D s_5_17: write-var walkstate.7 <= s_5_16
        fn_state.walkstate._7 = s_5_16;
        // D s_5_18: read-var walkstate:struct
        let s_5_18: ProductType96e7acababe246a1 = fn_state.walkstate;
        // N s_5_19: return s_5_18
        return s_5_18;
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_6_0: const #1u : u32
        let s_6_0: u32 = 1;
        // D s_6_1: write-var tablebase.1 <= s_6_0
        fn_state.tablebase._1 = s_6_0;
        // N s_6_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_7_0: read-var walkparams.24:struct
        let s_7_0: bool = fn_state.walkparams._24;
        // D s_7_1: cast zx s_7_0 -> bv
        let s_7_1: Bits = Bits::new(s_7_0 as u128, 1u16);
        // C s_7_2: const #0u : u8
        let s_7_2: bool = false;
        // C s_7_3: cast zx s_7_2 -> bv
        let s_7_3: Bits = Bits::new(s_7_2 as u128, 1u16);
        // D s_7_4: cmp-eq s_7_1 s_7_3
        let s_7_4: bool = ((s_7_1) == (s_7_3));
        // N s_7_5: branch s_7_4 b9 b8
        if s_7_4 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_8_0: const #0u : u32
        let s_8_0: u32 = 0;
        // D s_8_1: write-var tablebase.1 <= s_8_0
        fn_state.tablebase._1 = s_8_0;
        // N s_8_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_9_0: const #1u : u32
        let s_9_0: u32 = 1;
        // D s_9_1: write-var tablebase.1 <= s_9_0
        fn_state.tablebase._1 = s_9_0;
        // N s_9_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_10_0: const #22824u : u32
        let s_10_0: u32 = 22824;
        // D s_10_1: read-reg s_10_0:u64
        let s_10_1: u64 = {
            let value = state.read_register::<u64>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // C s_10_2: const #128s : i
        let s_10_2: i128 = 128;
        // D s_10_3: cast zx s_10_1 -> bv
        let s_10_3: Bits = Bits::new(s_10_1 as u128, 64u16);
        // D s_10_4: bits-cast zx s_10_3 -> bv length s_10_2
        let s_10_4: Bits = s_10_3.zero_extend(s_10_2);
        // D s_10_5: cast reint s_10_4 -> u128
        let s_10_5: u128 = (s_10_4.value() as u128);
        // D s_10_6: write-var ttbr <= s_10_5
        fn_state.ttbr = s_10_5;
        // N s_10_7: jump b2
        return block_2(state, tracer, fn_state);
    }
}
