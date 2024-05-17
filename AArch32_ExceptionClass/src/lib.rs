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
use ThisInstrLength::*;
use Unreachable::*;
use common::*;
pub fn AArch32_ExceptionClass<T: Tracer>(
    state: &mut State,
    tracer: &T,
    exceptype: u32,
) -> ProductTypec1bd230b943b3b8c {
    #[derive(Default)]
    struct FunctionState {
        gs_8838: bool,
        ecshadow_123: i128,
        gs_8837: bool,
        ec: i128,
        il: bool,
        il_is_valid: bool,
        exceptype: u32,
    }
    let fn_state = FunctionState {
        exceptype,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_0_0: const #1u : u8
        let s_0_0: bool = true;
        // D s_0_1: write-var il_is_valid <= s_0_0
        fn_state.il_is_valid = s_0_0;
        // C s_0_2: const #0u : u32
        let s_0_2: u32 = 0;
        // D s_0_3: read-var exceptype:u32
        let s_0_3: u32 = fn_state.exceptype;
        // D s_0_4: cmp-eq s_0_2 s_0_3
        let s_0_4: bool = ((s_0_2) == (s_0_3));
        // D s_0_5: not s_0_4
        let s_0_5: bool = !s_0_4;
        // N s_0_6: branch s_0_5 b18 b1
        if s_0_5 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_1_0: const #0u : u8
        let s_1_0: u8 = 0;
        // C s_1_1: cast zx s_1_0 -> bv
        let s_1_1: Bits = Bits::new(s_1_0 as u128, 8u16);
        // C s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (s_1_1.value() as i128);
        // D s_1_3: write-var ec <= s_1_2
        fn_state.ec = s_1_2;
        // C s_1_4: const #0u : u8
        let s_1_4: bool = false;
        // D s_1_5: write-var il_is_valid <= s_1_4
        fn_state.il_is_valid = s_1_4;
        // N s_1_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_2_0: const #32u : u8
        let s_2_0: u8 = 32;
        // C s_2_1: cast zx s_2_0 -> bv
        let s_2_1: Bits = Bits::new(s_2_0 as u128, 8u16);
        // C s_2_2: cast zx s_2_1 -> i
        let s_2_2: i128 = (s_2_1.value() as i128);
        // C s_2_3: cast reint s_2_2 -> i64
        let s_2_3: i64 = (s_2_2 as i64);
        // C s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: read-var ec:i
        let s_2_5: i128 = fn_state.ec;
        // D s_2_6: cmp-eq s_2_5 s_2_4
        let s_2_6: bool = ((s_2_5) == (s_2_4));
        // N s_2_7: branch s_2_6 b17 b3
        if s_2_6 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_3_0: const #36u : u8
        let s_3_0: u8 = 36;
        // C s_3_1: cast zx s_3_0 -> bv
        let s_3_1: Bits = Bits::new(s_3_0 as u128, 8u16);
        // C s_3_2: cast zx s_3_1 -> i
        let s_3_2: i128 = (s_3_1.value() as i128);
        // C s_3_3: cast reint s_3_2 -> i64
        let s_3_3: i64 = (s_3_2 as i64);
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // D s_3_5: read-var ec:i
        let s_3_5: i128 = fn_state.ec;
        // D s_3_6: cmp-eq s_3_5 s_3_4
        let s_3_6: bool = ((s_3_5) == (s_3_4));
        // D s_3_7: write-var gs#8837 <= s_3_6
        fn_state.gs_8837 = s_3_6;
        // N s_3_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_4_0: read-var gs#8837:u8
        let s_4_0: bool = fn_state.gs_8837;
        // N s_4_1: branch s_4_0 b16 b5
        if s_4_0 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#8838 <= s_5_0
        fn_state.gs_8838 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_6_0: read-var gs#8838:u8
        let s_6_0: bool = fn_state.gs_8838;
        // N s_6_1: branch s_6_0 b15 b7
        if s_6_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_8_0: read-var ec:i
        let s_8_0: i128 = fn_state.ec;
        // D s_8_1: write-var ecshadow#123 <= s_8_0
        fn_state.ecshadow_123 = s_8_0;
        // D s_8_2: read-var il_is_valid:u8
        let s_8_2: bool = fn_state.il_is_valid;
        // N s_8_3: branch s_8_2 b11 b9
        if s_8_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_9_0: const #1u : u8
        let s_9_0: bool = true;
        // D s_9_1: write-var il <= s_9_0
        fn_state.il = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // D s_10_0: read-var ecshadow#123:i
        let s_10_0: i128 = fn_state.ecshadow_123;
        // D s_10_1: read-var il:u8
        let s_10_1: bool = fn_state.il;
        // D s_10_2: create-product struct = ["s_10_0", "s_10_1"]
        let s_10_2: ProductTypec1bd230b943b3b8c = ProductTypec1bd230b943b3b8c {
            _0: s_10_0,
            _1: s_10_1,
        };
        // N s_10_3: return s_10_2
        return s_10_2;
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call ThisInstrLength(s_11_0)
        let s_11_1: i128 = ThisInstrLength(state, tracer, s_11_0);
        // C s_11_2: const #32s : i
        let s_11_2: i128 = 32;
        // S s_11_3: cmp-eq s_11_1 s_11_2
        let s_11_3: bool = ((s_11_1) == (s_11_2));
        // N s_11_4: branch s_11_3 b14 b12
        if s_11_3 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var il <= s_12_0
        fn_state.il = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // N s_13_0: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var il <= s_14_0
        fn_state.il = s_14_0;
        // N s_14_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_15_0: const #1s : i
        let s_15_0: i128 = 1;
        // D s_15_1: read-var ec:i
        let s_15_1: i128 = fn_state.ec;
        // D s_15_2: add s_15_1 s_15_0
        let s_15_2: i128 = (s_15_1 + s_15_0);
        // D s_15_3: write-var ec <= s_15_2
        fn_state.ec = s_15_2;
        // N s_15_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_16_0: const #16975u : u32
        let s_16_0: u32 = 16975;
        // D s_16_1: read-reg s_16_0:u8
        let s_16_1: u8 = {
            let value = state.read_register::<u8>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 2u16);
        // C s_16_3: const #432u : u32
        let s_16_3: u32 = 432;
        // D s_16_4: read-reg s_16_3:u8
        let s_16_4: u8 = {
            let value = state.read_register::<u8>(s_16_3 as isize);
            tracer.read_register(s_16_3 as isize, value);
            value
        };
        // D s_16_5: cast zx s_16_4 -> bv
        let s_16_5: Bits = Bits::new(s_16_4 as u128, 2u16);
        // D s_16_6: cmp-eq s_16_2 s_16_5
        let s_16_6: bool = ((s_16_2) == (s_16_5));
        // D s_16_7: write-var gs#8838 <= s_16_6
        fn_state.gs_8838 = s_16_6;
        // N s_16_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_17_0: const #1u : u8
        let s_17_0: bool = true;
        // D s_17_1: write-var gs#8837 <= s_17_0
        fn_state.gs_8837 = s_17_0;
        // N s_17_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_18_0: const #1u : u32
        let s_18_0: u32 = 1;
        // D s_18_1: read-var exceptype:u32
        let s_18_1: u32 = fn_state.exceptype;
        // D s_18_2: cmp-eq s_18_0 s_18_1
        let s_18_2: bool = ((s_18_0) == (s_18_1));
        // D s_18_3: not s_18_2
        let s_18_3: bool = !s_18_2;
        // N s_18_4: branch s_18_3 b20 b19
        if s_18_3 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_19_0: const #1u : u8
        let s_19_0: u8 = 1;
        // C s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 8u16);
        // C s_19_2: cast zx s_19_1 -> i
        let s_19_2: i128 = (s_19_1.value() as i128);
        // D s_19_3: write-var ec <= s_19_2
        fn_state.ec = s_19_2;
        // N s_19_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_20_0: const #2u : u32
        let s_20_0: u32 = 2;
        // D s_20_1: read-var exceptype:u32
        let s_20_1: u32 = fn_state.exceptype;
        // D s_20_2: cmp-eq s_20_0 s_20_1
        let s_20_2: bool = ((s_20_0) == (s_20_1));
        // D s_20_3: not s_20_2
        let s_20_3: bool = !s_20_2;
        // N s_20_4: branch s_20_3 b22 b21
        if s_20_3 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_21_0: const #3u : u8
        let s_21_0: u8 = 3;
        // C s_21_1: cast zx s_21_0 -> bv
        let s_21_1: Bits = Bits::new(s_21_0 as u128, 8u16);
        // C s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (s_21_1.value() as i128);
        // D s_21_3: write-var ec <= s_21_2
        fn_state.ec = s_21_2;
        // N s_21_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_22_0: const #3u : u32
        let s_22_0: u32 = 3;
        // D s_22_1: read-var exceptype:u32
        let s_22_1: u32 = fn_state.exceptype;
        // D s_22_2: cmp-eq s_22_0 s_22_1
        let s_22_2: bool = ((s_22_0) == (s_22_1));
        // D s_22_3: not s_22_2
        let s_22_3: bool = !s_22_2;
        // N s_22_4: branch s_22_3 b24 b23
        if s_22_3 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_23_0: const #4u : u8
        let s_23_0: u8 = 4;
        // C s_23_1: cast zx s_23_0 -> bv
        let s_23_1: Bits = Bits::new(s_23_0 as u128, 8u16);
        // C s_23_2: cast zx s_23_1 -> i
        let s_23_2: i128 = (s_23_1.value() as i128);
        // D s_23_3: write-var ec <= s_23_2
        fn_state.ec = s_23_2;
        // N s_23_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_24_0: const #4u : u32
        let s_24_0: u32 = 4;
        // D s_24_1: read-var exceptype:u32
        let s_24_1: u32 = fn_state.exceptype;
        // D s_24_2: cmp-eq s_24_0 s_24_1
        let s_24_2: bool = ((s_24_0) == (s_24_1));
        // D s_24_3: not s_24_2
        let s_24_3: bool = !s_24_2;
        // N s_24_4: branch s_24_3 b26 b25
        if s_24_3 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_25_0: const #5u : u8
        let s_25_0: u8 = 5;
        // C s_25_1: cast zx s_25_0 -> bv
        let s_25_1: Bits = Bits::new(s_25_0 as u128, 8u16);
        // C s_25_2: cast zx s_25_1 -> i
        let s_25_2: i128 = (s_25_1.value() as i128);
        // D s_25_3: write-var ec <= s_25_2
        fn_state.ec = s_25_2;
        // N s_25_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_26_0: const #5u : u32
        let s_26_0: u32 = 5;
        // D s_26_1: read-var exceptype:u32
        let s_26_1: u32 = fn_state.exceptype;
        // D s_26_2: cmp-eq s_26_0 s_26_1
        let s_26_2: bool = ((s_26_0) == (s_26_1));
        // D s_26_3: not s_26_2
        let s_26_3: bool = !s_26_2;
        // N s_26_4: branch s_26_3 b28 b27
        if s_26_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_27(state, tracer, fn_state);
        };
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_27_0: const #6u : u8
        let s_27_0: u8 = 6;
        // C s_27_1: cast zx s_27_0 -> bv
        let s_27_1: Bits = Bits::new(s_27_0 as u128, 8u16);
        // C s_27_2: cast zx s_27_1 -> i
        let s_27_2: i128 = (s_27_1.value() as i128);
        // D s_27_3: write-var ec <= s_27_2
        fn_state.ec = s_27_2;
        // N s_27_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_28_0: const #7u : u32
        let s_28_0: u32 = 7;
        // D s_28_1: read-var exceptype:u32
        let s_28_1: u32 = fn_state.exceptype;
        // D s_28_2: cmp-eq s_28_0 s_28_1
        let s_28_2: bool = ((s_28_0) == (s_28_1));
        // D s_28_3: not s_28_2
        let s_28_3: bool = !s_28_2;
        // N s_28_4: branch s_28_3 b30 b29
        if s_28_3 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_29_0: const #7u : u8
        let s_29_0: u8 = 7;
        // C s_29_1: cast zx s_29_0 -> bv
        let s_29_1: Bits = Bits::new(s_29_0 as u128, 8u16);
        // C s_29_2: cast zx s_29_1 -> i
        let s_29_2: i128 = (s_29_1.value() as i128);
        // D s_29_3: write-var ec <= s_29_2
        fn_state.ec = s_29_2;
        // N s_29_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_30_0: const #8u : u32
        let s_30_0: u32 = 8;
        // D s_30_1: read-var exceptype:u32
        let s_30_1: u32 = fn_state.exceptype;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // D s_30_3: not s_30_2
        let s_30_3: bool = !s_30_2;
        // N s_30_4: branch s_30_3 b32 b31
        if s_30_3 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_31_0: const #8u : u8
        let s_31_0: u8 = 8;
        // C s_31_1: cast zx s_31_0 -> bv
        let s_31_1: Bits = Bits::new(s_31_0 as u128, 8u16);
        // C s_31_2: cast zx s_31_1 -> i
        let s_31_2: i128 = (s_31_1.value() as i128);
        // D s_31_3: write-var ec <= s_31_2
        fn_state.ec = s_31_2;
        // N s_31_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_32_0: const #10u : u32
        let s_32_0: u32 = 10;
        // D s_32_1: read-var exceptype:u32
        let s_32_1: u32 = fn_state.exceptype;
        // D s_32_2: cmp-eq s_32_0 s_32_1
        let s_32_2: bool = ((s_32_0) == (s_32_1));
        // D s_32_3: not s_32_2
        let s_32_3: bool = !s_32_2;
        // N s_32_4: branch s_32_3 b34 b33
        if s_32_3 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_33_0: const #9u : u8
        let s_33_0: u8 = 9;
        // C s_33_1: cast zx s_33_0 -> bv
        let s_33_1: Bits = Bits::new(s_33_0 as u128, 8u16);
        // C s_33_2: cast zx s_33_1 -> i
        let s_33_2: i128 = (s_33_1.value() as i128);
        // D s_33_3: write-var ec <= s_33_2
        fn_state.ec = s_33_2;
        // N s_33_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_34_0: const #34u : u32
        let s_34_0: u32 = 34;
        // D s_34_1: read-var exceptype:u32
        let s_34_1: u32 = fn_state.exceptype;
        // D s_34_2: cmp-eq s_34_0 s_34_1
        let s_34_2: bool = ((s_34_0) == (s_34_1));
        // D s_34_3: not s_34_2
        let s_34_3: bool = !s_34_2;
        // N s_34_4: branch s_34_3 b36 b35
        if s_34_3 {
            return block_36(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_35_0: const #27u : u8
        let s_35_0: u8 = 27;
        // C s_35_1: cast zx s_35_0 -> bv
        let s_35_1: Bits = Bits::new(s_35_0 as u128, 8u16);
        // C s_35_2: cast zx s_35_1 -> i
        let s_35_2: i128 = (s_35_1.value() as i128);
        // D s_35_3: write-var ec <= s_35_2
        fn_state.ec = s_35_2;
        // N s_35_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_36_0: const #35u : u32
        let s_36_0: u32 = 35;
        // D s_36_1: read-var exceptype:u32
        let s_36_1: u32 = fn_state.exceptype;
        // D s_36_2: cmp-eq s_36_0 s_36_1
        let s_36_2: bool = ((s_36_0) == (s_36_1));
        // D s_36_3: not s_36_2
        let s_36_3: bool = !s_36_2;
        // N s_36_4: branch s_36_3 b38 b37
        if s_36_3 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_37_0: const #30u : u8
        let s_37_0: u8 = 30;
        // C s_37_1: cast zx s_37_0 -> bv
        let s_37_1: Bits = Bits::new(s_37_0 as u128, 8u16);
        // C s_37_2: cast zx s_37_1 -> i
        let s_37_2: i128 = (s_37_1.value() as i128);
        // D s_37_3: write-var ec <= s_37_2
        fn_state.ec = s_37_2;
        // N s_37_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_38_0: const #6u : u32
        let s_38_0: u32 = 6;
        // D s_38_1: read-var exceptype:u32
        let s_38_1: u32 = fn_state.exceptype;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // N s_38_4: branch s_38_3 b40 b39
        if s_38_3 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_39_0: const #12u : u8
        let s_39_0: u8 = 12;
        // C s_39_1: cast zx s_39_0 -> bv
        let s_39_1: Bits = Bits::new(s_39_0 as u128, 8u16);
        // C s_39_2: cast zx s_39_1 -> i
        let s_39_2: i128 = (s_39_1.value() as i128);
        // D s_39_3: write-var ec <= s_39_2
        fn_state.ec = s_39_2;
        // N s_39_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_40_0: const #36u : u32
        let s_40_0: u32 = 36;
        // D s_40_1: read-var exceptype:u32
        let s_40_1: u32 = fn_state.exceptype;
        // D s_40_2: cmp-eq s_40_0 s_40_1
        let s_40_2: bool = ((s_40_0) == (s_40_1));
        // D s_40_3: not s_40_2
        let s_40_3: bool = !s_40_2;
        // N s_40_4: branch s_40_3 b42 b41
        if s_40_3 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_41(state, tracer, fn_state);
        };
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_41_0: const #13u : u8
        let s_41_0: u8 = 13;
        // C s_41_1: cast zx s_41_0 -> bv
        let s_41_1: Bits = Bits::new(s_41_0 as u128, 8u16);
        // C s_41_2: cast zx s_41_1 -> i
        let s_41_2: i128 = (s_41_1.value() as i128);
        // D s_41_3: write-var ec <= s_41_2
        fn_state.ec = s_41_2;
        // N s_41_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_42_0: const #11u : u32
        let s_42_0: u32 = 11;
        // D s_42_1: read-var exceptype:u32
        let s_42_1: u32 = fn_state.exceptype;
        // D s_42_2: cmp-eq s_42_0 s_42_1
        let s_42_2: bool = ((s_42_0) == (s_42_1));
        // D s_42_3: not s_42_2
        let s_42_3: bool = !s_42_2;
        // N s_42_4: branch s_42_3 b44 b43
        if s_42_3 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_43_0: const #14u : u8
        let s_43_0: u8 = 14;
        // C s_43_1: cast zx s_43_0 -> bv
        let s_43_1: Bits = Bits::new(s_43_0 as u128, 8u16);
        // C s_43_2: cast zx s_43_1 -> i
        let s_43_2: i128 = (s_43_1.value() as i128);
        // D s_43_3: write-var ec <= s_43_2
        fn_state.ec = s_43_2;
        // C s_43_4: const #0u : u8
        let s_43_4: bool = false;
        // D s_43_5: write-var il_is_valid <= s_43_4
        fn_state.il_is_valid = s_43_4;
        // N s_43_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_44_0: const #12u : u32
        let s_44_0: u32 = 12;
        // D s_44_1: read-var exceptype:u32
        let s_44_1: u32 = fn_state.exceptype;
        // D s_44_2: cmp-eq s_44_0 s_44_1
        let s_44_2: bool = ((s_44_0) == (s_44_1));
        // D s_44_3: not s_44_2
        let s_44_3: bool = !s_44_2;
        // N s_44_4: branch s_44_3 b46 b45
        if s_44_3 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_45(state, tracer, fn_state);
        };
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_45_0: const #17u : u8
        let s_45_0: u8 = 17;
        // C s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 8u16);
        // C s_45_2: cast zx s_45_1 -> i
        let s_45_2: i128 = (s_45_1.value() as i128);
        // D s_45_3: write-var ec <= s_45_2
        fn_state.ec = s_45_2;
        // N s_45_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_46_0: const #13u : u32
        let s_46_0: u32 = 13;
        // D s_46_1: read-var exceptype:u32
        let s_46_1: u32 = fn_state.exceptype;
        // D s_46_2: cmp-eq s_46_0 s_46_1
        let s_46_2: bool = ((s_46_0) == (s_46_1));
        // D s_46_3: not s_46_2
        let s_46_3: bool = !s_46_2;
        // N s_46_4: branch s_46_3 b48 b47
        if s_46_3 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_47(state, tracer, fn_state);
        };
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_47_0: const #18u : u8
        let s_47_0: u8 = 18;
        // C s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 8u16);
        // C s_47_2: cast zx s_47_1 -> i
        let s_47_2: i128 = (s_47_1.value() as i128);
        // D s_47_3: write-var ec <= s_47_2
        fn_state.ec = s_47_2;
        // N s_47_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_48_0: const #14u : u32
        let s_48_0: u32 = 14;
        // D s_48_1: read-var exceptype:u32
        let s_48_1: u32 = fn_state.exceptype;
        // D s_48_2: cmp-eq s_48_0 s_48_1
        let s_48_2: bool = ((s_48_0) == (s_48_1));
        // D s_48_3: not s_48_2
        let s_48_3: bool = !s_48_2;
        // N s_48_4: branch s_48_3 b50 b49
        if s_48_3 {
            return block_50(state, tracer, fn_state);
        } else {
            return block_49(state, tracer, fn_state);
        };
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_49_0: const #19u : u8
        let s_49_0: u8 = 19;
        // C s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 8u16);
        // C s_49_2: cast zx s_49_1 -> i
        let s_49_2: i128 = (s_49_1.value() as i128);
        // D s_49_3: write-var ec <= s_49_2
        fn_state.ec = s_49_2;
        // N s_49_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_50_0: const #17u : u32
        let s_50_0: u32 = 17;
        // D s_50_1: read-var exceptype:u32
        let s_50_1: u32 = fn_state.exceptype;
        // D s_50_2: cmp-eq s_50_0 s_50_1
        let s_50_2: bool = ((s_50_0) == (s_50_1));
        // D s_50_3: not s_50_2
        let s_50_3: bool = !s_50_2;
        // N s_50_4: branch s_50_3 b52 b51
        if s_50_3 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_51_0: const #32u : u8
        let s_51_0: u8 = 32;
        // C s_51_1: cast zx s_51_0 -> bv
        let s_51_1: Bits = Bits::new(s_51_0 as u128, 8u16);
        // C s_51_2: cast zx s_51_1 -> i
        let s_51_2: i128 = (s_51_1.value() as i128);
        // D s_51_3: write-var ec <= s_51_2
        fn_state.ec = s_51_2;
        // C s_51_4: const #0u : u8
        let s_51_4: bool = false;
        // D s_51_5: write-var il_is_valid <= s_51_4
        fn_state.il_is_valid = s_51_4;
        // N s_51_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_52_0: const #18u : u32
        let s_52_0: u32 = 18;
        // D s_52_1: read-var exceptype:u32
        let s_52_1: u32 = fn_state.exceptype;
        // D s_52_2: cmp-eq s_52_0 s_52_1
        let s_52_2: bool = ((s_52_0) == (s_52_1));
        // D s_52_3: not s_52_2
        let s_52_3: bool = !s_52_2;
        // N s_52_4: branch s_52_3 b54 b53
        if s_52_3 {
            return block_54(state, tracer, fn_state);
        } else {
            return block_53(state, tracer, fn_state);
        };
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_53_0: const #34u : u8
        let s_53_0: u8 = 34;
        // C s_53_1: cast zx s_53_0 -> bv
        let s_53_1: Bits = Bits::new(s_53_0 as u128, 8u16);
        // C s_53_2: cast zx s_53_1 -> i
        let s_53_2: i128 = (s_53_1.value() as i128);
        // D s_53_3: write-var ec <= s_53_2
        fn_state.ec = s_53_2;
        // C s_53_4: const #0u : u8
        let s_53_4: bool = false;
        // D s_53_5: write-var il_is_valid <= s_53_4
        fn_state.il_is_valid = s_53_4;
        // N s_53_6: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_54_0: const #19u : u32
        let s_54_0: u32 = 19;
        // D s_54_1: read-var exceptype:u32
        let s_54_1: u32 = fn_state.exceptype;
        // D s_54_2: cmp-eq s_54_0 s_54_1
        let s_54_2: bool = ((s_54_0) == (s_54_1));
        // D s_54_3: not s_54_2
        let s_54_3: bool = !s_54_2;
        // N s_54_4: branch s_54_3 b56 b55
        if s_54_3 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_55_0: const #36u : u8
        let s_55_0: u8 = 36;
        // C s_55_1: cast zx s_55_0 -> bv
        let s_55_1: Bits = Bits::new(s_55_0 as u128, 8u16);
        // C s_55_2: cast zx s_55_1 -> i
        let s_55_2: i128 = (s_55_1.value() as i128);
        // D s_55_3: write-var ec <= s_55_2
        fn_state.ec = s_55_2;
        // N s_55_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_56_0: const #20u : u32
        let s_56_0: u32 = 20;
        // D s_56_1: read-var exceptype:u32
        let s_56_1: u32 = fn_state.exceptype;
        // D s_56_2: cmp-eq s_56_0 s_56_1
        let s_56_2: bool = ((s_56_0) == (s_56_1));
        // D s_56_3: not s_56_2
        let s_56_3: bool = !s_56_2;
        // N s_56_4: branch s_56_3 b58 b57
        if s_56_3 {
            return block_58(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_57_0: const #37u : u8
        let s_57_0: u8 = 37;
        // C s_57_1: cast zx s_57_0 -> bv
        let s_57_1: Bits = Bits::new(s_57_0 as u128, 8u16);
        // C s_57_2: cast zx s_57_1 -> i
        let s_57_2: i128 = (s_57_1.value() as i128);
        // D s_57_3: write-var ec <= s_57_2
        fn_state.ec = s_57_2;
        // N s_57_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_58_0: const #23u : u32
        let s_58_0: u32 = 23;
        // D s_58_1: read-var exceptype:u32
        let s_58_1: u32 = fn_state.exceptype;
        // D s_58_2: cmp-eq s_58_0 s_58_1
        let s_58_2: bool = ((s_58_0) == (s_58_1));
        // D s_58_3: not s_58_2
        let s_58_3: bool = !s_58_2;
        // N s_58_4: branch s_58_3 b60 b59
        if s_58_3 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_59(state, tracer, fn_state);
        };
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_59_0: const #40u : u8
        let s_59_0: u8 = 40;
        // C s_59_1: cast zx s_59_0 -> bv
        let s_59_1: Bits = Bits::new(s_59_0 as u128, 8u16);
        // C s_59_2: cast zx s_59_1 -> i
        let s_59_2: i128 = (s_59_1.value() as i128);
        // D s_59_3: write-var ec <= s_59_2
        fn_state.ec = s_59_2;
        // N s_59_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypec1bd230b943b3b8c {
        // C s_60_0: const #() : ()
        let s_60_0: () = ();
        // S s_60_1: call Unreachable(s_60_0)
        let s_60_1: () = Unreachable(state, tracer, s_60_0);
        // N s_60_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
