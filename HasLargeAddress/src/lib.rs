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
use Unreachable::*;
use Have52BitIPAAndPASpaceExt::*;
use common::*;
pub fn HasLargeAddress<T: Tracer>(state: &mut State, tracer: &T, regime: u32) -> bool {
    #[derive(Default)]
    struct FunctionState {
        ga_20738: bool,
        return_value: bool,
        regime: u32,
    }
    let fn_state = FunctionState {
        regime,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call Have52BitIPAAndPASpaceExt(s_0_0)
        let s_0_1: bool = Have52BitIPAAndPASpaceExt(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b11 b1
        if s_0_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // D s_1_1: read-var regime:u32
        let s_1_1: u32 = fn_state.regime;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // D s_1_3: not s_1_2
        let s_1_3: bool = !s_1_2;
        // N s_1_4: branch s_1_3 b4 b2
        if s_1_3 {
            return block_4(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_2_0: const #10736u : u32
        let s_2_0: u32 = 10736;
        // D s_2_1: read-reg s_2_0:u64
        let s_2_1: u64 = {
            let value = state.read_register::<u64>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // C s_2_2: const #32s : i
        let s_2_2: i128 = 32;
        // D s_2_3: cast zx s_2_1 -> bv
        let s_2_3: Bits = Bits::new(s_2_1 as u128, 64u16);
        // C s_2_4: const #1u : u64
        let s_2_4: u64 = 1;
        // D s_2_5: bit-extract s_2_3 s_2_2 s_2_4
        let s_2_5: Bits = (Bits::new(
            ((s_2_3) >> (s_2_2)).value(),
            u16::try_from(s_2_4).unwrap(),
        ));
        // D s_2_6: cast reint s_2_5 -> u8
        let s_2_6: bool = ((s_2_5.value()) != 0);
        // C s_2_7: const #0s : i
        let s_2_7: i128 = 0;
        // C s_2_8: const #0u : u64
        let s_2_8: u64 = 0;
        // D s_2_9: cast zx s_2_6 -> u64
        let s_2_9: u64 = (s_2_6 as u64);
        // C s_2_10: const #1u : u64
        let s_2_10: u64 = 1;
        // D s_2_11: and s_2_9 s_2_10
        let s_2_11: u64 = ((s_2_9) & (s_2_10));
        // D s_2_12: cmp-eq s_2_11 s_2_10
        let s_2_12: bool = ((s_2_11) == (s_2_10));
        // D s_2_13: lsl s_2_9 s_2_7
        let s_2_13: u64 = s_2_9 << s_2_7;
        // D s_2_14: or s_2_8 s_2_13
        let s_2_14: u64 = ((s_2_8) | (s_2_13));
        // D s_2_15: cmpl s_2_13
        let s_2_15: u64 = !s_2_13;
        // D s_2_16: and s_2_8 s_2_15
        let s_2_16: u64 = ((s_2_8) & (s_2_15));
        // D s_2_17: select s_2_12 s_2_14 s_2_16
        let s_2_17: u64 = if s_2_12 { s_2_14 } else { s_2_16 };
        // D s_2_18: cast trunc s_2_17 -> u8
        let s_2_18: bool = ((s_2_17) != 0);
        // D s_2_19: cast zx s_2_18 -> bv
        let s_2_19: Bits = Bits::new(s_2_18 as u128, 1u16);
        // C s_2_20: const #1u : u8
        let s_2_20: bool = true;
        // C s_2_21: cast zx s_2_20 -> bv
        let s_2_21: Bits = Bits::new(s_2_20 as u128, 1u16);
        // D s_2_22: cmp-eq s_2_19 s_2_21
        let s_2_22: bool = ((s_2_19) == (s_2_21));
        // D s_2_23: write-var return_value <= s_2_22
        fn_state.return_value = s_2_22;
        // N s_2_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_3_0: read-var return_value:u8
        let s_3_0: bool = fn_state.return_value;
        // N s_3_1: return s_3_0
        return s_3_0;
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_4_0: const #2u : u32
        let s_4_0: u32 = 2;
        // D s_4_1: read-var regime:u32
        let s_4_1: u32 = fn_state.regime;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // D s_4_3: not s_4_2
        let s_4_3: bool = !s_4_2;
        // N s_4_4: branch s_4_3 b6 b5
        if s_4_3 {
            return block_6(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #12816u : u32
        let s_5_0: u32 = 12816;
        // D s_5_1: read-reg s_5_0:u64
        let s_5_1: u64 = {
            let value = state.read_register::<u64>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // C s_5_2: const #32s : i
        let s_5_2: i128 = 32;
        // D s_5_3: cast zx s_5_1 -> bv
        let s_5_3: Bits = Bits::new(s_5_1 as u128, 64u16);
        // C s_5_4: const #1u : u64
        let s_5_4: u64 = 1;
        // D s_5_5: bit-extract s_5_3 s_5_2 s_5_4
        let s_5_5: Bits = (Bits::new(
            ((s_5_3) >> (s_5_2)).value(),
            u16::try_from(s_5_4).unwrap(),
        ));
        // D s_5_6: cast reint s_5_5 -> u8
        let s_5_6: bool = ((s_5_5.value()) != 0);
        // C s_5_7: const #0s : i
        let s_5_7: i128 = 0;
        // C s_5_8: const #0u : u64
        let s_5_8: u64 = 0;
        // D s_5_9: cast zx s_5_6 -> u64
        let s_5_9: u64 = (s_5_6 as u64);
        // C s_5_10: const #1u : u64
        let s_5_10: u64 = 1;
        // D s_5_11: and s_5_9 s_5_10
        let s_5_11: u64 = ((s_5_9) & (s_5_10));
        // D s_5_12: cmp-eq s_5_11 s_5_10
        let s_5_12: bool = ((s_5_11) == (s_5_10));
        // D s_5_13: lsl s_5_9 s_5_7
        let s_5_13: u64 = s_5_9 << s_5_7;
        // D s_5_14: or s_5_8 s_5_13
        let s_5_14: u64 = ((s_5_8) | (s_5_13));
        // D s_5_15: cmpl s_5_13
        let s_5_15: u64 = !s_5_13;
        // D s_5_16: and s_5_8 s_5_15
        let s_5_16: u64 = ((s_5_8) & (s_5_15));
        // D s_5_17: select s_5_12 s_5_14 s_5_16
        let s_5_17: u64 = if s_5_12 { s_5_14 } else { s_5_16 };
        // D s_5_18: cast trunc s_5_17 -> u8
        let s_5_18: bool = ((s_5_17) != 0);
        // D s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 1u16);
        // C s_5_20: const #1u : u8
        let s_5_20: bool = true;
        // C s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 1u16);
        // D s_5_22: cmp-eq s_5_19 s_5_21
        let s_5_22: bool = ((s_5_19) == (s_5_21));
        // D s_5_23: write-var return_value <= s_5_22
        fn_state.return_value = s_5_22;
        // N s_5_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_6_0: const #3u : u32
        let s_6_0: u32 = 3;
        // D s_6_1: read-var regime:u32
        let s_6_1: u32 = fn_state.regime;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b8 b7
        if s_6_3 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_7_0: const #12816u : u32
        let s_7_0: u32 = 12816;
        // D s_7_1: read-reg s_7_0:u64
        let s_7_1: u64 = {
            let value = state.read_register::<u64>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // C s_7_2: const #59s : i
        let s_7_2: i128 = 59;
        // D s_7_3: cast zx s_7_1 -> bv
        let s_7_3: Bits = Bits::new(s_7_1 as u128, 64u16);
        // C s_7_4: const #1u : u64
        let s_7_4: u64 = 1;
        // D s_7_5: bit-extract s_7_3 s_7_2 s_7_4
        let s_7_5: Bits = (Bits::new(
            ((s_7_3) >> (s_7_2)).value(),
            u16::try_from(s_7_4).unwrap(),
        ));
        // D s_7_6: cast reint s_7_5 -> u8
        let s_7_6: bool = ((s_7_5.value()) != 0);
        // C s_7_7: const #0s : i
        let s_7_7: i128 = 0;
        // C s_7_8: const #0u : u64
        let s_7_8: u64 = 0;
        // D s_7_9: cast zx s_7_6 -> u64
        let s_7_9: u64 = (s_7_6 as u64);
        // C s_7_10: const #1u : u64
        let s_7_10: u64 = 1;
        // D s_7_11: and s_7_9 s_7_10
        let s_7_11: u64 = ((s_7_9) & (s_7_10));
        // D s_7_12: cmp-eq s_7_11 s_7_10
        let s_7_12: bool = ((s_7_11) == (s_7_10));
        // D s_7_13: lsl s_7_9 s_7_7
        let s_7_13: u64 = s_7_9 << s_7_7;
        // D s_7_14: or s_7_8 s_7_13
        let s_7_14: u64 = ((s_7_8) | (s_7_13));
        // D s_7_15: cmpl s_7_13
        let s_7_15: u64 = !s_7_13;
        // D s_7_16: and s_7_8 s_7_15
        let s_7_16: u64 = ((s_7_8) & (s_7_15));
        // D s_7_17: select s_7_12 s_7_14 s_7_16
        let s_7_17: u64 = if s_7_12 { s_7_14 } else { s_7_16 };
        // D s_7_18: cast trunc s_7_17 -> u8
        let s_7_18: bool = ((s_7_17) != 0);
        // D s_7_19: cast zx s_7_18 -> bv
        let s_7_19: Bits = Bits::new(s_7_18 as u128, 1u16);
        // C s_7_20: const #1u : u8
        let s_7_20: bool = true;
        // C s_7_21: cast zx s_7_20 -> bv
        let s_7_21: Bits = Bits::new(s_7_20 as u128, 1u16);
        // D s_7_22: cmp-eq s_7_19 s_7_21
        let s_7_22: bool = ((s_7_19) == (s_7_21));
        // D s_7_23: write-var return_value <= s_7_22
        fn_state.return_value = s_7_22;
        // N s_7_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_8_0: const #4u : u32
        let s_8_0: u32 = 4;
        // D s_8_1: read-var regime:u32
        let s_8_1: u32 = fn_state.regime;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // D s_8_3: not s_8_2
        let s_8_3: bool = !s_8_2;
        // N s_8_4: branch s_8_3 b10 b9
        if s_8_3 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #22392u : u32
        let s_9_0: u32 = 22392;
        // D s_9_1: read-reg s_9_0:u64
        let s_9_1: u64 = {
            let value = state.read_register::<u64>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // C s_9_2: const #59s : i
        let s_9_2: i128 = 59;
        // D s_9_3: cast zx s_9_1 -> bv
        let s_9_3: Bits = Bits::new(s_9_1 as u128, 64u16);
        // C s_9_4: const #1u : u64
        let s_9_4: u64 = 1;
        // D s_9_5: bit-extract s_9_3 s_9_2 s_9_4
        let s_9_5: Bits = (Bits::new(
            ((s_9_3) >> (s_9_2)).value(),
            u16::try_from(s_9_4).unwrap(),
        ));
        // D s_9_6: cast reint s_9_5 -> u8
        let s_9_6: bool = ((s_9_5.value()) != 0);
        // C s_9_7: const #0s : i
        let s_9_7: i128 = 0;
        // C s_9_8: const #0u : u64
        let s_9_8: u64 = 0;
        // D s_9_9: cast zx s_9_6 -> u64
        let s_9_9: u64 = (s_9_6 as u64);
        // C s_9_10: const #1u : u64
        let s_9_10: u64 = 1;
        // D s_9_11: and s_9_9 s_9_10
        let s_9_11: u64 = ((s_9_9) & (s_9_10));
        // D s_9_12: cmp-eq s_9_11 s_9_10
        let s_9_12: bool = ((s_9_11) == (s_9_10));
        // D s_9_13: lsl s_9_9 s_9_7
        let s_9_13: u64 = s_9_9 << s_9_7;
        // D s_9_14: or s_9_8 s_9_13
        let s_9_14: u64 = ((s_9_8) | (s_9_13));
        // D s_9_15: cmpl s_9_13
        let s_9_15: u64 = !s_9_13;
        // D s_9_16: and s_9_8 s_9_15
        let s_9_16: u64 = ((s_9_8) & (s_9_15));
        // D s_9_17: select s_9_12 s_9_14 s_9_16
        let s_9_17: u64 = if s_9_12 { s_9_14 } else { s_9_16 };
        // D s_9_18: cast trunc s_9_17 -> u8
        let s_9_18: bool = ((s_9_17) != 0);
        // D s_9_19: cast zx s_9_18 -> bv
        let s_9_19: Bits = Bits::new(s_9_18 as u128, 1u16);
        // C s_9_20: const #1u : u8
        let s_9_20: bool = true;
        // C s_9_21: cast zx s_9_20 -> bv
        let s_9_21: Bits = Bits::new(s_9_20 as u128, 1u16);
        // D s_9_22: cmp-eq s_9_19 s_9_21
        let s_9_22: bool = ((s_9_19) == (s_9_21));
        // D s_9_23: write-var return_value <= s_9_22
        fn_state.return_value = s_9_22;
        // N s_9_24: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call Unreachable(s_10_0)
        let s_10_1: () = Unreachable(state, tracer, s_10_0);
        // D s_10_2: read-var ga#20738:u8
        let s_10_2: bool = fn_state.ga_20738;
        // D s_10_3: write-var return_value <= s_10_2
        fn_state.return_value = s_10_2;
        // N s_10_4: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_11_0: const #0u : u8
        let s_11_0: bool = false;
        // D s_11_1: write-var return_value <= s_11_0
        fn_state.return_value = s_11_0;
        // N s_11_2: jump b3
        return block_3(state, tracer, fn_state);
    }
}
