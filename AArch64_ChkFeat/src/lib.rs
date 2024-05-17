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
use Bit::*;
use GCSEnabled::*;
use Zeros::*;
use HaveGCS::*;
use common::*;
pub fn AArch64_ChkFeat<T: Tracer>(
    state: &mut State,
    tracer: &T,
    feat_select: u64,
) -> u64 {
    #[derive(Default)]
    struct FunctionState {
        ga_4590: bool,
        feat_en: u64,
        gs_6796: bool,
        feat_select: u64,
    }
    let fn_state = FunctionState {
        feat_select,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_0_0: const #64s : i
        let s_0_0: i128 = 64;
        // S s_0_1: call Zeros(s_0_0)
        let s_0_1: Bits = Zeros(state, tracer, s_0_0);
        // S s_0_2: cast reint s_0_1 -> u64
        let s_0_2: u64 = (s_0_1.value() as u64);
        // D s_0_3: write-var feat_en <= s_0_2
        fn_state.feat_en = s_0_2;
        // C s_0_4: const #() : ()
        let s_0_4: () = ();
        // S s_0_5: call HaveGCS(s_0_4)
        let s_0_5: bool = HaveGCS(state, tracer, s_0_4);
        // N s_0_6: branch s_0_5 b6 b1
        if s_0_5 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#6796 <= s_1_0
        fn_state.gs_6796 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_2_0: read-var gs#6796:u8
        let s_2_0: bool = fn_state.gs_6796;
        // N s_2_1: branch s_2_0 b5 b3
        if s_2_0 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var ga#4590 <= s_3_0
        fn_state.ga_4590 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // D s_4_0: read-var ga#4590:u8
        let s_4_0: bool = fn_state.ga_4590;
        // D s_4_1: call Bit(s_4_0)
        let s_4_1: bool = Bit(state, tracer, s_4_0);
        // C s_4_2: const #0s : i
        let s_4_2: i128 = 0;
        // D s_4_3: read-var feat_en:u64
        let s_4_3: u64 = fn_state.feat_en;
        // D s_4_4: cast zx s_4_3 -> bv
        let s_4_4: Bits = Bits::new(s_4_3 as u128, 64u16);
        // C s_4_5: const #1u : u64
        let s_4_5: u64 = 1;
        // D s_4_6: bit-insert s_4_4 s_4_4 s_4_2 s_4_5
        let s_4_6: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_4_5 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_4_4.length(),
            );
            (s_4_4 & mask) | (s_4_4 << s_4_2)
        };
        // D s_4_7: cast reint s_4_6 -> u64
        let s_4_7: u64 = (s_4_6.value() as u64);
        // D s_4_8: write-var feat_en <= s_4_7
        fn_state.feat_en = s_4_7;
        // D s_4_9: read-var feat_en:u64
        let s_4_9: u64 = fn_state.feat_en;
        // D s_4_10: cast zx s_4_9 -> bv
        let s_4_10: Bits = Bits::new(s_4_9 as u128, 64u16);
        // D s_4_11: not s_4_10
        let s_4_11: Bits = !s_4_10;
        // D s_4_12: cast reint s_4_11 -> u64
        let s_4_12: u64 = (s_4_11.value() as u64);
        // D s_4_13: read-var feat_select:u64
        let s_4_13: u64 = fn_state.feat_select;
        // D s_4_14: cast zx s_4_13 -> bv
        let s_4_14: Bits = Bits::new(s_4_13 as u128, 64u16);
        // D s_4_15: cast zx s_4_12 -> bv
        let s_4_15: Bits = Bits::new(s_4_12 as u128, 64u16);
        // D s_4_16: and s_4_14 s_4_15
        let s_4_16: Bits = ((s_4_14) & (s_4_15));
        // D s_4_17: cast reint s_4_16 -> u64
        let s_4_17: u64 = (s_4_16.value() as u64);
        // N s_4_18: return s_4_17
        return s_4_17;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_5_0: const #1u : u8
        let s_5_0: bool = true;
        // D s_5_1: write-var ga#4590 <= s_5_0
        fn_state.ga_4590 = s_5_0;
        // N s_5_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u64 {
        // C s_6_0: const #16975u : u32
        let s_6_0: u32 = 16975;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call GCSEnabled(s_6_1)
        let s_6_2: bool = GCSEnabled(state, tracer, s_6_1);
        // D s_6_3: write-var gs#6796 <= s_6_2
        fn_state.gs_6796 = s_6_2;
        // N s_6_4: jump b2
        return block_2(state, tracer, fn_state);
    }
}
