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
use u__UNKNOWN_bits::*;
use LookUpRIndex::*;
use ExceptionSyndrome::*;
use integer_subrange::*;
use Bit::*;
use ConditionSyndrome::*;
use Zeros::*;
use Unreachable::*;
use common::*;
pub fn AArch64_AArch32SystemAccessTrapSyndrome<T: Tracer>(
    state: &mut State,
    tracer: &T,
    instr: u32,
    ec: i128,
) -> ProductTypeb7f99f96751e17c4 {
    #[derive(Default)]
    struct FunctionState {
        gs_25032: bool,
        iss: u32,
        gs_24935: bool,
        gs_25037: bool,
        gs_24934: bool,
        gs_24936: bool,
        except: ProductTypeb7f99f96751e17c4,
        return_value: ProductTypeb7f99f96751e17c4,
        gs_24937: bool,
        instr: u32,
        ec: i128,
    }
    let fn_state = FunctionState {
        instr,
        ec,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_0_0: read-var ec:i
        let s_0_0: i128 = fn_state.ec;
        // C s_0_1: const #0s : i
        let s_0_1: i128 = 0;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // D s_0_3: not s_0_2
        let s_0_3: bool = !s_0_2;
        // N s_0_4: branch s_0_3 b47 b1
        if s_0_3 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_1_0: const #0u : u32
        let s_1_0: u32 = 0;
        // S s_1_1: call ExceptionSyndrome(s_1_0)
        let s_1_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(state, tracer, s_1_0);
        // D s_1_2: write-var except <= s_1_1
        fn_state.except = s_1_1;
        // N s_1_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_2_0: const #20s : i
        let s_2_0: i128 = 20;
        // S s_2_1: call Zeros(s_2_0)
        let s_2_1: Bits = Zeros(state, tracer, s_2_0);
        // S s_2_2: cast reint s_2_1 -> u20
        let s_2_2: u32 = (s_2_1.value() as u32);
        // D s_2_3: write-var iss <= s_2_2
        fn_state.iss = s_2_2;
        // D s_2_4: read-var except.1:struct
        let s_2_4: u32 = fn_state.except._1;
        // C s_2_5: const #0u : u32
        let s_2_5: u32 = 0;
        // D s_2_6: cmp-eq s_2_4 s_2_5
        let s_2_6: bool = ((s_2_4) == (s_2_5));
        // N s_2_7: branch s_2_6 b46 b3
        if s_2_6 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_3_0: read-var except.1:struct
        let s_3_0: u32 = fn_state.except._1;
        // C s_3_1: const #8u : u32
        let s_3_1: u32 = 8;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b45 b4
        if s_3_2 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_4_0: read-var except.1:struct
        let s_4_0: u32 = fn_state.except._1;
        // C s_4_1: const #4u : u32
        let s_4_1: u32 = 4;
        // D s_4_2: cmp-eq s_4_0 s_4_1
        let s_4_2: bool = ((s_4_0) == (s_4_1));
        // N s_4_3: branch s_4_2 b44 b5
        if s_4_2 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_5_0: read-var except.1:struct
        let s_5_0: u32 = fn_state.except._1;
        // C s_5_1: const #2u : u32
        let s_5_1: u32 = 2;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: write-var gs#24934 <= s_5_2
        fn_state.gs_24934 = s_5_2;
        // N s_5_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_6_0: read-var gs#24934:u8
        let s_6_0: bool = fn_state.gs_24934;
        // D s_6_1: write-var gs#24935 <= s_6_0
        fn_state.gs_24935 = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_7_0: read-var gs#24935:u8
        let s_7_0: bool = fn_state.gs_24935;
        // N s_7_1: branch s_7_0 b30 b8
        if s_7_0 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_8_0: read-var except.1:struct
        let s_8_0: u32 = fn_state.except._1;
        // C s_8_1: const #6u : u32
        let s_8_1: u32 = 6;
        // D s_8_2: cmp-eq s_8_0 s_8_1
        let s_8_2: bool = ((s_8_0) == (s_8_1));
        // N s_8_3: branch s_8_2 b29 b9
        if s_8_2 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_9_0: read-var except.1:struct
        let s_9_0: u32 = fn_state.except._1;
        // C s_9_1: const #7u : u32
        let s_9_1: u32 = 7;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // N s_9_3: branch s_9_2 b28 b10
        if s_9_2 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_10_0: read-var except.1:struct
        let s_10_0: u32 = fn_state.except._1;
        // C s_10_1: const #3u : u32
        let s_10_1: u32 = 3;
        // D s_10_2: cmp-eq s_10_0 s_10_1
        let s_10_2: bool = ((s_10_0) == (s_10_1));
        // D s_10_3: write-var gs#24936 <= s_10_2
        fn_state.gs_24936 = s_10_2;
        // N s_10_4: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_11_0: read-var gs#24936:u8
        let s_11_0: bool = fn_state.gs_24936;
        // D s_11_1: write-var gs#24937 <= s_11_0
        fn_state.gs_24937 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_12_0: read-var gs#24937:u8
        let s_12_0: bool = fn_state.gs_24937;
        // N s_12_1: branch s_12_0 b21 b13
        if s_12_0 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_13_0: read-var except.1:struct
        let s_13_0: u32 = fn_state.except._1;
        // C s_13_1: const #5u : u32
        let s_13_1: u32 = 5;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // N s_13_3: branch s_13_2 b17 b14
        if s_13_2 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // N s_14_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_15_0: const #20s : i
        let s_15_0: i128 = 20;
        // D s_15_1: read-var instr:u32
        let s_15_1: u32 = fn_state.instr;
        // D s_15_2: cast zx s_15_1 -> bv
        let s_15_2: Bits = Bits::new(s_15_1 as u128, 32u16);
        // C s_15_3: const #1u : u64
        let s_15_3: u64 = 1;
        // D s_15_4: bit-extract s_15_2 s_15_0 s_15_3
        let s_15_4: Bits = (Bits::new(
            ((s_15_2) >> (s_15_0)).value(),
            u16::try_from(s_15_3).unwrap(),
        ));
        // D s_15_5: cast reint s_15_4 -> u8
        let s_15_5: bool = ((s_15_4.value()) != 0);
        // C s_15_6: const #0s : i
        let s_15_6: i128 = 0;
        // C s_15_7: const #0u : u64
        let s_15_7: u64 = 0;
        // D s_15_8: cast zx s_15_5 -> u64
        let s_15_8: u64 = (s_15_5 as u64);
        // C s_15_9: const #1u : u64
        let s_15_9: u64 = 1;
        // D s_15_10: and s_15_8 s_15_9
        let s_15_10: u64 = ((s_15_8) & (s_15_9));
        // D s_15_11: cmp-eq s_15_10 s_15_9
        let s_15_11: bool = ((s_15_10) == (s_15_9));
        // D s_15_12: lsl s_15_8 s_15_6
        let s_15_12: u64 = s_15_8 << s_15_6;
        // D s_15_13: or s_15_7 s_15_12
        let s_15_13: u64 = ((s_15_7) | (s_15_12));
        // D s_15_14: cmpl s_15_12
        let s_15_14: u64 = !s_15_12;
        // D s_15_15: and s_15_7 s_15_14
        let s_15_15: u64 = ((s_15_7) & (s_15_14));
        // D s_15_16: select s_15_11 s_15_13 s_15_15
        let s_15_16: u64 = if s_15_11 { s_15_13 } else { s_15_15 };
        // D s_15_17: cast trunc s_15_16 -> u8
        let s_15_17: bool = ((s_15_16) != 0);
        // D s_15_18: call Bit(s_15_17)
        let s_15_18: bool = Bit(state, tracer, s_15_17);
        // C s_15_19: const #0s : i
        let s_15_19: i128 = 0;
        // D s_15_20: read-var iss:u20
        let s_15_20: u32 = fn_state.iss;
        // D s_15_21: cast zx s_15_20 -> bv
        let s_15_21: Bits = Bits::new(s_15_20 as u128, 20u16);
        // C s_15_22: const #1u : u64
        let s_15_22: u64 = 1;
        // D s_15_23: bit-insert s_15_21 s_15_21 s_15_19 s_15_22
        let s_15_23: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_15_22 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_15_21.length(),
            );
            (s_15_21 & mask) | (s_15_21 << s_15_19)
        };
        // D s_15_24: cast reint s_15_23 -> u20
        let s_15_24: u32 = (s_15_23.value() as u32);
        // D s_15_25: write-var iss <= s_15_24
        fn_state.iss = s_15_24;
        // C s_15_26: const #() : ()
        let s_15_26: () = ();
        // S s_15_27: call ConditionSyndrome(s_15_26)
        let s_15_27: u8 = ConditionSyndrome(state, tracer, s_15_26);
        // D s_15_28: read-var except:struct
        let s_15_28: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_15_29: write-var except <= s_15_28
        fn_state.except = s_15_28;
        // D s_15_30: read-var except:struct
        let s_15_30: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_15_31: write-var except <= s_15_30
        fn_state.except = s_15_30;
        // D s_15_32: read-var except:struct
        let s_15_32: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_15_33: write-var return_value <= s_15_32
        fn_state.return_value = s_15_32;
        // N s_15_34: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_16_0: read-var return_value:struct
        let s_16_0: ProductTypeb7f99f96751e17c4 = fn_state.return_value;
        // N s_16_1: return s_16_0
        return s_16_0;
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_17_0: const #0s : i
        let s_17_0: i128 = 0;
        // D s_17_1: read-var instr:u32
        let s_17_1: u32 = fn_state.instr;
        // D s_17_2: cast zx s_17_1 -> bv
        let s_17_2: Bits = Bits::new(s_17_1 as u128, 32u16);
        // C s_17_3: const #1s : i64
        let s_17_3: i64 = 1;
        // C s_17_4: cast zx s_17_3 -> i
        let s_17_4: i128 = (i128::try_from(s_17_3).unwrap());
        // C s_17_5: const #7s : i
        let s_17_5: i128 = 7;
        // C s_17_6: add s_17_5 s_17_4
        let s_17_6: i128 = (s_17_5 + s_17_4);
        // D s_17_7: bit-extract s_17_2 s_17_0 s_17_6
        let s_17_7: Bits = (Bits::new(
            ((s_17_2) >> (s_17_0)).value(),
            u16::try_from(s_17_6).unwrap(),
        ));
        // D s_17_8: cast reint s_17_7 -> u8
        let s_17_8: u8 = (s_17_7.value() as u8);
        // C s_17_9: const #12s : i
        let s_17_9: i128 = 12;
        // D s_17_10: read-var iss:u20
        let s_17_10: u32 = fn_state.iss;
        // D s_17_11: cast zx s_17_10 -> bv
        let s_17_11: Bits = Bits::new(s_17_10 as u128, 20u16);
        // D s_17_12: cast zx s_17_8 -> bv
        let s_17_12: Bits = Bits::new(s_17_8 as u128, 8u16);
        // C s_17_13: const #7s : i
        let s_17_13: i128 = 7;
        // C s_17_14: const #1u : u64
        let s_17_14: u64 = 1;
        // C s_17_15: cast zx s_17_14 -> bv
        let s_17_15: Bits = Bits::new(s_17_14 as u128, 64u16);
        // C s_17_16: lsl s_17_15 s_17_13
        let s_17_16: Bits = s_17_15 << s_17_13;
        // C s_17_17: sub s_17_16 s_17_15
        let s_17_17: Bits = ((s_17_16) - (s_17_15));
        // D s_17_18: and s_17_12 s_17_17
        let s_17_18: Bits = ((s_17_12) & (s_17_17));
        // D s_17_19: lsl s_17_18 s_17_9
        let s_17_19: Bits = s_17_18 << s_17_9;
        // C s_17_20: lsl s_17_17 s_17_9
        let s_17_20: Bits = s_17_17 << s_17_9;
        // C s_17_21: cmpl s_17_20
        let s_17_21: Bits = !s_17_20;
        // D s_17_22: and s_17_11 s_17_21
        let s_17_22: Bits = ((s_17_11) & (s_17_21));
        // D s_17_23: or s_17_22 s_17_19
        let s_17_23: Bits = ((s_17_22) | (s_17_19));
        // D s_17_24: cast reint s_17_23 -> u20
        let s_17_24: u32 = (s_17_23.value() as u32);
        // D s_17_25: write-var iss <= s_17_24
        fn_state.iss = s_17_24;
        // C s_17_26: const #23s : i
        let s_17_26: i128 = 23;
        // D s_17_27: read-var instr:u32
        let s_17_27: u32 = fn_state.instr;
        // D s_17_28: cast zx s_17_27 -> bv
        let s_17_28: Bits = Bits::new(s_17_27 as u128, 32u16);
        // C s_17_29: const #1u : u64
        let s_17_29: u64 = 1;
        // D s_17_30: bit-extract s_17_28 s_17_26 s_17_29
        let s_17_30: Bits = (Bits::new(
            ((s_17_28) >> (s_17_26)).value(),
            u16::try_from(s_17_29).unwrap(),
        ));
        // D s_17_31: cast reint s_17_30 -> u8
        let s_17_31: bool = ((s_17_30.value()) != 0);
        // C s_17_32: const #0s : i
        let s_17_32: i128 = 0;
        // C s_17_33: const #0u : u64
        let s_17_33: u64 = 0;
        // D s_17_34: cast zx s_17_31 -> u64
        let s_17_34: u64 = (s_17_31 as u64);
        // C s_17_35: const #1u : u64
        let s_17_35: u64 = 1;
        // D s_17_36: and s_17_34 s_17_35
        let s_17_36: u64 = ((s_17_34) & (s_17_35));
        // D s_17_37: cmp-eq s_17_36 s_17_35
        let s_17_37: bool = ((s_17_36) == (s_17_35));
        // D s_17_38: lsl s_17_34 s_17_32
        let s_17_38: u64 = s_17_34 << s_17_32;
        // D s_17_39: or s_17_33 s_17_38
        let s_17_39: u64 = ((s_17_33) | (s_17_38));
        // D s_17_40: cmpl s_17_38
        let s_17_40: u64 = !s_17_38;
        // D s_17_41: and s_17_33 s_17_40
        let s_17_41: u64 = ((s_17_33) & (s_17_40));
        // D s_17_42: select s_17_37 s_17_39 s_17_41
        let s_17_42: u64 = if s_17_37 { s_17_39 } else { s_17_41 };
        // D s_17_43: cast trunc s_17_42 -> u8
        let s_17_43: bool = ((s_17_42) != 0);
        // D s_17_44: call Bit(s_17_43)
        let s_17_44: bool = Bit(state, tracer, s_17_43);
        // C s_17_45: const #4s : i
        let s_17_45: i128 = 4;
        // D s_17_46: read-var iss:u20
        let s_17_46: u32 = fn_state.iss;
        // D s_17_47: cast zx s_17_46 -> bv
        let s_17_47: Bits = Bits::new(s_17_46 as u128, 20u16);
        // C s_17_48: const #1u : u64
        let s_17_48: u64 = 1;
        // D s_17_49: bit-insert s_17_47 s_17_47 s_17_45 s_17_48
        let s_17_49: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_17_48 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_17_47.length(),
            );
            (s_17_47 & mask) | (s_17_47 << s_17_45)
        };
        // D s_17_50: cast reint s_17_49 -> u20
        let s_17_50: u32 = (s_17_49.value() as u32);
        // D s_17_51: write-var iss <= s_17_50
        fn_state.iss = s_17_50;
        // C s_17_52: const #24s : i
        let s_17_52: i128 = 24;
        // D s_17_53: read-var instr:u32
        let s_17_53: u32 = fn_state.instr;
        // D s_17_54: cast zx s_17_53 -> bv
        let s_17_54: Bits = Bits::new(s_17_53 as u128, 32u16);
        // C s_17_55: const #1u : u64
        let s_17_55: u64 = 1;
        // D s_17_56: bit-extract s_17_54 s_17_52 s_17_55
        let s_17_56: Bits = (Bits::new(
            ((s_17_54) >> (s_17_52)).value(),
            u16::try_from(s_17_55).unwrap(),
        ));
        // D s_17_57: cast reint s_17_56 -> u8
        let s_17_57: bool = ((s_17_56.value()) != 0);
        // C s_17_58: const #0s : i
        let s_17_58: i128 = 0;
        // C s_17_59: const #0u : u64
        let s_17_59: u64 = 0;
        // D s_17_60: cast zx s_17_57 -> u64
        let s_17_60: u64 = (s_17_57 as u64);
        // C s_17_61: const #1u : u64
        let s_17_61: u64 = 1;
        // D s_17_62: and s_17_60 s_17_61
        let s_17_62: u64 = ((s_17_60) & (s_17_61));
        // D s_17_63: cmp-eq s_17_62 s_17_61
        let s_17_63: bool = ((s_17_62) == (s_17_61));
        // D s_17_64: lsl s_17_60 s_17_58
        let s_17_64: u64 = s_17_60 << s_17_58;
        // D s_17_65: or s_17_59 s_17_64
        let s_17_65: u64 = ((s_17_59) | (s_17_64));
        // D s_17_66: cmpl s_17_64
        let s_17_66: u64 = !s_17_64;
        // D s_17_67: and s_17_59 s_17_66
        let s_17_67: u64 = ((s_17_59) & (s_17_66));
        // D s_17_68: select s_17_63 s_17_65 s_17_67
        let s_17_68: u64 = if s_17_63 { s_17_65 } else { s_17_67 };
        // D s_17_69: cast trunc s_17_68 -> u8
        let s_17_69: bool = ((s_17_68) != 0);
        // C s_17_70: const #21s : i
        let s_17_70: i128 = 21;
        // D s_17_71: read-var instr:u32
        let s_17_71: u32 = fn_state.instr;
        // D s_17_72: cast zx s_17_71 -> bv
        let s_17_72: Bits = Bits::new(s_17_71 as u128, 32u16);
        // C s_17_73: const #1u : u64
        let s_17_73: u64 = 1;
        // D s_17_74: bit-extract s_17_72 s_17_70 s_17_73
        let s_17_74: Bits = (Bits::new(
            ((s_17_72) >> (s_17_70)).value(),
            u16::try_from(s_17_73).unwrap(),
        ));
        // D s_17_75: cast reint s_17_74 -> u8
        let s_17_75: bool = ((s_17_74.value()) != 0);
        // C s_17_76: const #0s : i
        let s_17_76: i128 = 0;
        // C s_17_77: const #0u : u64
        let s_17_77: u64 = 0;
        // D s_17_78: cast zx s_17_75 -> u64
        let s_17_78: u64 = (s_17_75 as u64);
        // C s_17_79: const #1u : u64
        let s_17_79: u64 = 1;
        // D s_17_80: and s_17_78 s_17_79
        let s_17_80: u64 = ((s_17_78) & (s_17_79));
        // D s_17_81: cmp-eq s_17_80 s_17_79
        let s_17_81: bool = ((s_17_80) == (s_17_79));
        // D s_17_82: lsl s_17_78 s_17_76
        let s_17_82: u64 = s_17_78 << s_17_76;
        // D s_17_83: or s_17_77 s_17_82
        let s_17_83: u64 = ((s_17_77) | (s_17_82));
        // D s_17_84: cmpl s_17_82
        let s_17_84: u64 = !s_17_82;
        // D s_17_85: and s_17_77 s_17_84
        let s_17_85: u64 = ((s_17_77) & (s_17_84));
        // D s_17_86: select s_17_81 s_17_83 s_17_85
        let s_17_86: u64 = if s_17_81 { s_17_83 } else { s_17_85 };
        // D s_17_87: cast trunc s_17_86 -> u8
        let s_17_87: bool = ((s_17_86) != 0);
        // D s_17_88: cast zx s_17_69 -> bv
        let s_17_88: Bits = Bits::new(s_17_69 as u128, 1u16);
        // D s_17_89: cast zx s_17_87 -> bv
        let s_17_89: Bits = Bits::new(s_17_87 as u128, 1u16);
        // D s_17_90: cast reint s_17_88 -> u128
        let s_17_90: u128 = (s_17_88.value() as u128);
        // D s_17_91: size-of s_17_88
        let s_17_91: u16 = s_17_88.length();
        // D s_17_92: cast reint s_17_89 -> u128
        let s_17_92: u128 = (s_17_89.value() as u128);
        // D s_17_93: size-of s_17_89
        let s_17_93: u16 = s_17_89.length();
        // D s_17_94: lsl s_17_90 s_17_93
        let s_17_94: u128 = s_17_90 << s_17_93;
        // D s_17_95: or s_17_94 s_17_92
        let s_17_95: u128 = ((s_17_94) | (s_17_92));
        // D s_17_96: add s_17_91 s_17_93
        let s_17_96: u16 = (s_17_91 + s_17_93);
        // D s_17_97: create-bits s_17_95 s_17_96
        let s_17_97: Bits = Bits::new(s_17_95, s_17_96);
        // D s_17_98: cast reint s_17_97 -> u8
        let s_17_98: u8 = (s_17_97.value() as u8);
        // C s_17_99: const #1s : i
        let s_17_99: i128 = 1;
        // D s_17_100: read-var iss:u20
        let s_17_100: u32 = fn_state.iss;
        // D s_17_101: cast zx s_17_100 -> bv
        let s_17_101: Bits = Bits::new(s_17_100 as u128, 20u16);
        // D s_17_102: cast zx s_17_98 -> bv
        let s_17_102: Bits = Bits::new(s_17_98 as u128, 2u16);
        // C s_17_103: const #1s : i
        let s_17_103: i128 = 1;
        // C s_17_104: const #1u : u64
        let s_17_104: u64 = 1;
        // C s_17_105: cast zx s_17_104 -> bv
        let s_17_105: Bits = Bits::new(s_17_104 as u128, 64u16);
        // C s_17_106: lsl s_17_105 s_17_103
        let s_17_106: Bits = s_17_105 << s_17_103;
        // C s_17_107: sub s_17_106 s_17_105
        let s_17_107: Bits = ((s_17_106) - (s_17_105));
        // D s_17_108: and s_17_102 s_17_107
        let s_17_108: Bits = ((s_17_102) & (s_17_107));
        // D s_17_109: lsl s_17_108 s_17_99
        let s_17_109: Bits = s_17_108 << s_17_99;
        // C s_17_110: lsl s_17_107 s_17_99
        let s_17_110: Bits = s_17_107 << s_17_99;
        // C s_17_111: cmpl s_17_110
        let s_17_111: Bits = !s_17_110;
        // D s_17_112: and s_17_101 s_17_111
        let s_17_112: Bits = ((s_17_101) & (s_17_111));
        // D s_17_113: or s_17_112 s_17_109
        let s_17_113: Bits = ((s_17_112) | (s_17_109));
        // D s_17_114: cast reint s_17_113 -> u20
        let s_17_114: u32 = (s_17_113.value() as u32);
        // D s_17_115: write-var iss <= s_17_114
        fn_state.iss = s_17_114;
        // C s_17_116: const #16s : i
        let s_17_116: i128 = 16;
        // D s_17_117: read-var instr:u32
        let s_17_117: u32 = fn_state.instr;
        // D s_17_118: cast zx s_17_117 -> bv
        let s_17_118: Bits = Bits::new(s_17_117 as u128, 32u16);
        // C s_17_119: const #1s : i64
        let s_17_119: i64 = 1;
        // C s_17_120: cast zx s_17_119 -> i
        let s_17_120: i128 = (i128::try_from(s_17_119).unwrap());
        // C s_17_121: const #3s : i
        let s_17_121: i128 = 3;
        // C s_17_122: add s_17_121 s_17_120
        let s_17_122: i128 = (s_17_121 + s_17_120);
        // D s_17_123: bit-extract s_17_118 s_17_116 s_17_122
        let s_17_123: Bits = (Bits::new(
            ((s_17_118) >> (s_17_116)).value(),
            u16::try_from(s_17_122).unwrap(),
        ));
        // D s_17_124: cast reint s_17_123 -> u8
        let s_17_124: u8 = (s_17_123.value() as u8);
        // D s_17_125: cast zx s_17_124 -> bv
        let s_17_125: Bits = Bits::new(s_17_124 as u128, 4u16);
        // C s_17_126: const #15u : u8
        let s_17_126: u8 = 15;
        // C s_17_127: cast zx s_17_126 -> bv
        let s_17_127: Bits = Bits::new(s_17_126 as u128, 4u16);
        // D s_17_128: cmp-eq s_17_125 s_17_127
        let s_17_128: bool = ((s_17_125) == (s_17_127));
        // N s_17_129: branch s_17_128 b20 b18
        if s_17_128 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // N s_19_0: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_20_0: const #5s : i64
        let s_20_0: i64 = 5;
        // C s_20_1: cast zx s_20_0 -> i
        let s_20_1: i128 = (i128::try_from(s_20_0).unwrap());
        // S s_20_2: call __UNKNOWN_bits(s_20_1)
        let s_20_2: Bits = u__UNKNOWN_bits(state, tracer, s_20_1);
        // S s_20_3: cast reint s_20_2 -> u8
        let s_20_3: u8 = (s_20_2.value() as u8);
        // C s_20_4: const #5s : i
        let s_20_4: i128 = 5;
        // D s_20_5: read-var iss:u20
        let s_20_5: u32 = fn_state.iss;
        // D s_20_6: cast zx s_20_5 -> bv
        let s_20_6: Bits = Bits::new(s_20_5 as u128, 20u16);
        // S s_20_7: cast zx s_20_3 -> bv
        let s_20_7: Bits = Bits::new(s_20_3 as u128, 5u16);
        // C s_20_8: const #4s : i
        let s_20_8: i128 = 4;
        // C s_20_9: const #1u : u64
        let s_20_9: u64 = 1;
        // C s_20_10: cast zx s_20_9 -> bv
        let s_20_10: Bits = Bits::new(s_20_9 as u128, 64u16);
        // C s_20_11: lsl s_20_10 s_20_8
        let s_20_11: Bits = s_20_10 << s_20_8;
        // C s_20_12: sub s_20_11 s_20_10
        let s_20_12: Bits = ((s_20_11) - (s_20_10));
        // S s_20_13: and s_20_7 s_20_12
        let s_20_13: Bits = ((s_20_7) & (s_20_12));
        // S s_20_14: lsl s_20_13 s_20_4
        let s_20_14: Bits = s_20_13 << s_20_4;
        // C s_20_15: lsl s_20_12 s_20_4
        let s_20_15: Bits = s_20_12 << s_20_4;
        // C s_20_16: cmpl s_20_15
        let s_20_16: Bits = !s_20_15;
        // D s_20_17: and s_20_6 s_20_16
        let s_20_17: Bits = ((s_20_6) & (s_20_16));
        // D s_20_18: or s_20_17 s_20_14
        let s_20_18: Bits = ((s_20_17) | (s_20_14));
        // D s_20_19: cast reint s_20_18 -> u20
        let s_20_19: u32 = (s_20_18.value() as u32);
        // D s_20_20: write-var iss <= s_20_19
        fn_state.iss = s_20_19;
        // C s_20_21: const #1u : u8
        let s_20_21: bool = true;
        // S s_20_22: call Bit(s_20_21)
        let s_20_22: bool = Bit(state, tracer, s_20_21);
        // C s_20_23: const #3s : i
        let s_20_23: i128 = 3;
        // D s_20_24: read-var iss:u20
        let s_20_24: u32 = fn_state.iss;
        // D s_20_25: cast zx s_20_24 -> bv
        let s_20_25: Bits = Bits::new(s_20_24 as u128, 20u16);
        // C s_20_26: const #1u : u64
        let s_20_26: u64 = 1;
        // D s_20_27: bit-insert s_20_25 s_20_25 s_20_23 s_20_26
        let s_20_27: Bits = {
            let mask = !Bits::new(
                ((1u128).checked_shl(s_20_26 as u32).map(|x| x - 1).unwrap_or(!0)),
                s_20_25.length(),
            );
            (s_20_25 & mask) | (s_20_25 << s_20_23)
        };
        // D s_20_28: cast reint s_20_27 -> u20
        let s_20_28: u32 = (s_20_27.value() as u32);
        // D s_20_29: write-var iss <= s_20_28
        fn_state.iss = s_20_28;
        // N s_20_30: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_21_0: const #4s : i
        let s_21_0: i128 = 4;
        // D s_21_1: read-var instr:u32
        let s_21_1: u32 = fn_state.instr;
        // D s_21_2: cast zx s_21_1 -> bv
        let s_21_2: Bits = Bits::new(s_21_1 as u128, 32u16);
        // C s_21_3: const #1s : i64
        let s_21_3: i64 = 1;
        // C s_21_4: cast zx s_21_3 -> i
        let s_21_4: i128 = (i128::try_from(s_21_3).unwrap());
        // C s_21_5: const #3s : i
        let s_21_5: i128 = 3;
        // C s_21_6: add s_21_5 s_21_4
        let s_21_6: i128 = (s_21_5 + s_21_4);
        // D s_21_7: bit-extract s_21_2 s_21_0 s_21_6
        let s_21_7: Bits = (Bits::new(
            ((s_21_2) >> (s_21_0)).value(),
            u16::try_from(s_21_6).unwrap(),
        ));
        // D s_21_8: cast reint s_21_7 -> u8
        let s_21_8: u8 = (s_21_7.value() as u8);
        // C s_21_9: const #16s : i
        let s_21_9: i128 = 16;
        // D s_21_10: read-var iss:u20
        let s_21_10: u32 = fn_state.iss;
        // D s_21_11: cast zx s_21_10 -> bv
        let s_21_11: Bits = Bits::new(s_21_10 as u128, 20u16);
        // D s_21_12: cast zx s_21_8 -> bv
        let s_21_12: Bits = Bits::new(s_21_8 as u128, 4u16);
        // C s_21_13: const #3s : i
        let s_21_13: i128 = 3;
        // C s_21_14: const #1u : u64
        let s_21_14: u64 = 1;
        // C s_21_15: cast zx s_21_14 -> bv
        let s_21_15: Bits = Bits::new(s_21_14 as u128, 64u16);
        // C s_21_16: lsl s_21_15 s_21_13
        let s_21_16: Bits = s_21_15 << s_21_13;
        // C s_21_17: sub s_21_16 s_21_15
        let s_21_17: Bits = ((s_21_16) - (s_21_15));
        // D s_21_18: and s_21_12 s_21_17
        let s_21_18: Bits = ((s_21_12) & (s_21_17));
        // D s_21_19: lsl s_21_18 s_21_9
        let s_21_19: Bits = s_21_18 << s_21_9;
        // C s_21_20: lsl s_21_17 s_21_9
        let s_21_20: Bits = s_21_17 << s_21_9;
        // C s_21_21: cmpl s_21_20
        let s_21_21: Bits = !s_21_20;
        // D s_21_22: and s_21_11 s_21_21
        let s_21_22: Bits = ((s_21_11) & (s_21_21));
        // D s_21_23: or s_21_22 s_21_19
        let s_21_23: Bits = ((s_21_22) | (s_21_19));
        // D s_21_24: cast reint s_21_23 -> u20
        let s_21_24: u32 = (s_21_23.value() as u32);
        // D s_21_25: write-var iss <= s_21_24
        fn_state.iss = s_21_24;
        // C s_21_26: const #16s : i
        let s_21_26: i128 = 16;
        // D s_21_27: read-var instr:u32
        let s_21_27: u32 = fn_state.instr;
        // D s_21_28: cast zx s_21_27 -> bv
        let s_21_28: Bits = Bits::new(s_21_27 as u128, 32u16);
        // C s_21_29: const #1s : i64
        let s_21_29: i64 = 1;
        // C s_21_30: cast zx s_21_29 -> i
        let s_21_30: i128 = (i128::try_from(s_21_29).unwrap());
        // C s_21_31: const #3s : i
        let s_21_31: i128 = 3;
        // C s_21_32: add s_21_31 s_21_30
        let s_21_32: i128 = (s_21_31 + s_21_30);
        // D s_21_33: bit-extract s_21_28 s_21_26 s_21_32
        let s_21_33: Bits = (Bits::new(
            ((s_21_28) >> (s_21_26)).value(),
            u16::try_from(s_21_32).unwrap(),
        ));
        // D s_21_34: cast reint s_21_33 -> u8
        let s_21_34: u8 = (s_21_33.value() as u8);
        // D s_21_35: cast zx s_21_34 -> bv
        let s_21_35: Bits = Bits::new(s_21_34 as u128, 4u16);
        // C s_21_36: const #15u : u8
        let s_21_36: u8 = 15;
        // C s_21_37: cast zx s_21_36 -> bv
        let s_21_37: Bits = Bits::new(s_21_36 as u128, 4u16);
        // D s_21_38: cmp-eq s_21_35 s_21_37
        let s_21_38: bool = ((s_21_35) == (s_21_37));
        // N s_21_39: branch s_21_38 b27 b22
        if s_21_38 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_22_0: const #16s : i
        let s_22_0: i128 = 16;
        // D s_22_1: read-var instr:u32
        let s_22_1: u32 = fn_state.instr;
        // D s_22_2: cast zx s_22_1 -> bv
        let s_22_2: Bits = Bits::new(s_22_1 as u128, 32u16);
        // C s_22_3: const #1s : i64
        let s_22_3: i64 = 1;
        // C s_22_4: cast zx s_22_3 -> i
        let s_22_4: i128 = (i128::try_from(s_22_3).unwrap());
        // C s_22_5: const #3s : i
        let s_22_5: i128 = 3;
        // C s_22_6: add s_22_5 s_22_4
        let s_22_6: i128 = (s_22_5 + s_22_4);
        // D s_22_7: bit-extract s_22_2 s_22_0 s_22_6
        let s_22_7: Bits = (Bits::new(
            ((s_22_2) >> (s_22_0)).value(),
            u16::try_from(s_22_6).unwrap(),
        ));
        // D s_22_8: cast reint s_22_7 -> u8
        let s_22_8: u8 = (s_22_7.value() as u8);
        // D s_22_9: cast zx s_22_8 -> bv
        let s_22_9: Bits = Bits::new(s_22_8 as u128, 4u16);
        // D s_22_10: cast zx s_22_9 -> i
        let s_22_10: i128 = (s_22_9.value() as i128);
        // D s_22_11: cast reint s_22_10 -> i64
        let s_22_11: i64 = (s_22_10 as i64);
        // C s_22_12: const #16983u : u32
        let s_22_12: u32 = 16983;
        // D s_22_13: read-reg s_22_12:u8
        let s_22_13: u8 = {
            let value = state.read_register::<u8>(s_22_12 as isize);
            tracer.read_register(s_22_12 as isize, value);
            value
        };
        // D s_22_14: call LookUpRIndex(s_22_11, s_22_13)
        let s_22_14: i64 = LookUpRIndex(state, tracer, s_22_11, s_22_13);
        // C s_22_15: const #4s : i
        let s_22_15: i128 = 4;
        // C s_22_16: const #0s : i
        let s_22_16: i128 = 0;
        // D s_22_17: cast zx s_22_14 -> i
        let s_22_17: i128 = (i128::try_from(s_22_14).unwrap());
        // D s_22_18: call integer_subrange(s_22_17, s_22_15, s_22_16)
        let s_22_18: Bits = integer_subrange(state, tracer, s_22_17, s_22_15, s_22_16);
        // D s_22_19: cast reint s_22_18 -> u8
        let s_22_19: u8 = (s_22_18.value() as u8);
        // C s_22_20: const #10s : i
        let s_22_20: i128 = 10;
        // D s_22_21: read-var iss:u20
        let s_22_21: u32 = fn_state.iss;
        // D s_22_22: cast zx s_22_21 -> bv
        let s_22_22: Bits = Bits::new(s_22_21 as u128, 20u16);
        // D s_22_23: cast zx s_22_19 -> bv
        let s_22_23: Bits = Bits::new(s_22_19 as u128, 5u16);
        // C s_22_24: const #4s : i
        let s_22_24: i128 = 4;
        // C s_22_25: const #1u : u64
        let s_22_25: u64 = 1;
        // C s_22_26: cast zx s_22_25 -> bv
        let s_22_26: Bits = Bits::new(s_22_25 as u128, 64u16);
        // C s_22_27: lsl s_22_26 s_22_24
        let s_22_27: Bits = s_22_26 << s_22_24;
        // C s_22_28: sub s_22_27 s_22_26
        let s_22_28: Bits = ((s_22_27) - (s_22_26));
        // D s_22_29: and s_22_23 s_22_28
        let s_22_29: Bits = ((s_22_23) & (s_22_28));
        // D s_22_30: lsl s_22_29 s_22_20
        let s_22_30: Bits = s_22_29 << s_22_20;
        // C s_22_31: lsl s_22_28 s_22_20
        let s_22_31: Bits = s_22_28 << s_22_20;
        // C s_22_32: cmpl s_22_31
        let s_22_32: Bits = !s_22_31;
        // D s_22_33: and s_22_22 s_22_32
        let s_22_33: Bits = ((s_22_22) & (s_22_32));
        // D s_22_34: or s_22_33 s_22_30
        let s_22_34: Bits = ((s_22_33) | (s_22_30));
        // D s_22_35: cast reint s_22_34 -> u20
        let s_22_35: u32 = (s_22_34.value() as u32);
        // D s_22_36: write-var iss <= s_22_35
        fn_state.iss = s_22_35;
        // N s_22_37: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_23_0: const #12s : i
        let s_23_0: i128 = 12;
        // D s_23_1: read-var instr:u32
        let s_23_1: u32 = fn_state.instr;
        // D s_23_2: cast zx s_23_1 -> bv
        let s_23_2: Bits = Bits::new(s_23_1 as u128, 32u16);
        // C s_23_3: const #1s : i64
        let s_23_3: i64 = 1;
        // C s_23_4: cast zx s_23_3 -> i
        let s_23_4: i128 = (i128::try_from(s_23_3).unwrap());
        // C s_23_5: const #3s : i
        let s_23_5: i128 = 3;
        // C s_23_6: add s_23_5 s_23_4
        let s_23_6: i128 = (s_23_5 + s_23_4);
        // D s_23_7: bit-extract s_23_2 s_23_0 s_23_6
        let s_23_7: Bits = (Bits::new(
            ((s_23_2) >> (s_23_0)).value(),
            u16::try_from(s_23_6).unwrap(),
        ));
        // D s_23_8: cast reint s_23_7 -> u8
        let s_23_8: u8 = (s_23_7.value() as u8);
        // D s_23_9: cast zx s_23_8 -> bv
        let s_23_9: Bits = Bits::new(s_23_8 as u128, 4u16);
        // C s_23_10: const #15u : u8
        let s_23_10: u8 = 15;
        // C s_23_11: cast zx s_23_10 -> bv
        let s_23_11: Bits = Bits::new(s_23_10 as u128, 4u16);
        // D s_23_12: cmp-eq s_23_9 s_23_11
        let s_23_12: bool = ((s_23_9) == (s_23_11));
        // N s_23_13: branch s_23_12 b26 b24
        if s_23_12 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_24_0: const #12s : i
        let s_24_0: i128 = 12;
        // D s_24_1: read-var instr:u32
        let s_24_1: u32 = fn_state.instr;
        // D s_24_2: cast zx s_24_1 -> bv
        let s_24_2: Bits = Bits::new(s_24_1 as u128, 32u16);
        // C s_24_3: const #1s : i64
        let s_24_3: i64 = 1;
        // C s_24_4: cast zx s_24_3 -> i
        let s_24_4: i128 = (i128::try_from(s_24_3).unwrap());
        // C s_24_5: const #3s : i
        let s_24_5: i128 = 3;
        // C s_24_6: add s_24_5 s_24_4
        let s_24_6: i128 = (s_24_5 + s_24_4);
        // D s_24_7: bit-extract s_24_2 s_24_0 s_24_6
        let s_24_7: Bits = (Bits::new(
            ((s_24_2) >> (s_24_0)).value(),
            u16::try_from(s_24_6).unwrap(),
        ));
        // D s_24_8: cast reint s_24_7 -> u8
        let s_24_8: u8 = (s_24_7.value() as u8);
        // D s_24_9: cast zx s_24_8 -> bv
        let s_24_9: Bits = Bits::new(s_24_8 as u128, 4u16);
        // D s_24_10: cast zx s_24_9 -> i
        let s_24_10: i128 = (s_24_9.value() as i128);
        // D s_24_11: cast reint s_24_10 -> i64
        let s_24_11: i64 = (s_24_10 as i64);
        // C s_24_12: const #16983u : u32
        let s_24_12: u32 = 16983;
        // D s_24_13: read-reg s_24_12:u8
        let s_24_13: u8 = {
            let value = state.read_register::<u8>(s_24_12 as isize);
            tracer.read_register(s_24_12 as isize, value);
            value
        };
        // D s_24_14: call LookUpRIndex(s_24_11, s_24_13)
        let s_24_14: i64 = LookUpRIndex(state, tracer, s_24_11, s_24_13);
        // C s_24_15: const #4s : i
        let s_24_15: i128 = 4;
        // C s_24_16: const #0s : i
        let s_24_16: i128 = 0;
        // D s_24_17: cast zx s_24_14 -> i
        let s_24_17: i128 = (i128::try_from(s_24_14).unwrap());
        // D s_24_18: call integer_subrange(s_24_17, s_24_15, s_24_16)
        let s_24_18: Bits = integer_subrange(state, tracer, s_24_17, s_24_15, s_24_16);
        // D s_24_19: cast reint s_24_18 -> u8
        let s_24_19: u8 = (s_24_18.value() as u8);
        // C s_24_20: const #5s : i
        let s_24_20: i128 = 5;
        // D s_24_21: read-var iss:u20
        let s_24_21: u32 = fn_state.iss;
        // D s_24_22: cast zx s_24_21 -> bv
        let s_24_22: Bits = Bits::new(s_24_21 as u128, 20u16);
        // D s_24_23: cast zx s_24_19 -> bv
        let s_24_23: Bits = Bits::new(s_24_19 as u128, 5u16);
        // C s_24_24: const #4s : i
        let s_24_24: i128 = 4;
        // C s_24_25: const #1u : u64
        let s_24_25: u64 = 1;
        // C s_24_26: cast zx s_24_25 -> bv
        let s_24_26: Bits = Bits::new(s_24_25 as u128, 64u16);
        // C s_24_27: lsl s_24_26 s_24_24
        let s_24_27: Bits = s_24_26 << s_24_24;
        // C s_24_28: sub s_24_27 s_24_26
        let s_24_28: Bits = ((s_24_27) - (s_24_26));
        // D s_24_29: and s_24_23 s_24_28
        let s_24_29: Bits = ((s_24_23) & (s_24_28));
        // D s_24_30: lsl s_24_29 s_24_20
        let s_24_30: Bits = s_24_29 << s_24_20;
        // C s_24_31: lsl s_24_28 s_24_20
        let s_24_31: Bits = s_24_28 << s_24_20;
        // C s_24_32: cmpl s_24_31
        let s_24_32: Bits = !s_24_31;
        // D s_24_33: and s_24_22 s_24_32
        let s_24_33: Bits = ((s_24_22) & (s_24_32));
        // D s_24_34: or s_24_33 s_24_30
        let s_24_34: Bits = ((s_24_33) | (s_24_30));
        // D s_24_35: cast reint s_24_34 -> u20
        let s_24_35: u32 = (s_24_34.value() as u32);
        // D s_24_36: write-var iss <= s_24_35
        fn_state.iss = s_24_35;
        // N s_24_37: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_25_0: const #0s : i
        let s_25_0: i128 = 0;
        // D s_25_1: read-var instr:u32
        let s_25_1: u32 = fn_state.instr;
        // D s_25_2: cast zx s_25_1 -> bv
        let s_25_2: Bits = Bits::new(s_25_1 as u128, 32u16);
        // C s_25_3: const #1s : i64
        let s_25_3: i64 = 1;
        // C s_25_4: cast zx s_25_3 -> i
        let s_25_4: i128 = (i128::try_from(s_25_3).unwrap());
        // C s_25_5: const #3s : i
        let s_25_5: i128 = 3;
        // C s_25_6: add s_25_5 s_25_4
        let s_25_6: i128 = (s_25_5 + s_25_4);
        // D s_25_7: bit-extract s_25_2 s_25_0 s_25_6
        let s_25_7: Bits = (Bits::new(
            ((s_25_2) >> (s_25_0)).value(),
            u16::try_from(s_25_6).unwrap(),
        ));
        // D s_25_8: cast reint s_25_7 -> u8
        let s_25_8: u8 = (s_25_7.value() as u8);
        // C s_25_9: const #1s : i
        let s_25_9: i128 = 1;
        // D s_25_10: read-var iss:u20
        let s_25_10: u32 = fn_state.iss;
        // D s_25_11: cast zx s_25_10 -> bv
        let s_25_11: Bits = Bits::new(s_25_10 as u128, 20u16);
        // D s_25_12: cast zx s_25_8 -> bv
        let s_25_12: Bits = Bits::new(s_25_8 as u128, 4u16);
        // C s_25_13: const #3s : i
        let s_25_13: i128 = 3;
        // C s_25_14: const #1u : u64
        let s_25_14: u64 = 1;
        // C s_25_15: cast zx s_25_14 -> bv
        let s_25_15: Bits = Bits::new(s_25_14 as u128, 64u16);
        // C s_25_16: lsl s_25_15 s_25_13
        let s_25_16: Bits = s_25_15 << s_25_13;
        // C s_25_17: sub s_25_16 s_25_15
        let s_25_17: Bits = ((s_25_16) - (s_25_15));
        // D s_25_18: and s_25_12 s_25_17
        let s_25_18: Bits = ((s_25_12) & (s_25_17));
        // D s_25_19: lsl s_25_18 s_25_9
        let s_25_19: Bits = s_25_18 << s_25_9;
        // C s_25_20: lsl s_25_17 s_25_9
        let s_25_20: Bits = s_25_17 << s_25_9;
        // C s_25_21: cmpl s_25_20
        let s_25_21: Bits = !s_25_20;
        // D s_25_22: and s_25_11 s_25_21
        let s_25_22: Bits = ((s_25_11) & (s_25_21));
        // D s_25_23: or s_25_22 s_25_19
        let s_25_23: Bits = ((s_25_22) | (s_25_19));
        // D s_25_24: cast reint s_25_23 -> u20
        let s_25_24: u32 = (s_25_23.value() as u32);
        // D s_25_25: write-var iss <= s_25_24
        fn_state.iss = s_25_24;
        // N s_25_26: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_26_0: const #5s : i64
        let s_26_0: i64 = 5;
        // C s_26_1: cast zx s_26_0 -> i
        let s_26_1: i128 = (i128::try_from(s_26_0).unwrap());
        // S s_26_2: call __UNKNOWN_bits(s_26_1)
        let s_26_2: Bits = u__UNKNOWN_bits(state, tracer, s_26_1);
        // S s_26_3: cast reint s_26_2 -> u8
        let s_26_3: u8 = (s_26_2.value() as u8);
        // C s_26_4: const #5s : i
        let s_26_4: i128 = 5;
        // D s_26_5: read-var iss:u20
        let s_26_5: u32 = fn_state.iss;
        // D s_26_6: cast zx s_26_5 -> bv
        let s_26_6: Bits = Bits::new(s_26_5 as u128, 20u16);
        // S s_26_7: cast zx s_26_3 -> bv
        let s_26_7: Bits = Bits::new(s_26_3 as u128, 5u16);
        // C s_26_8: const #4s : i
        let s_26_8: i128 = 4;
        // C s_26_9: const #1u : u64
        let s_26_9: u64 = 1;
        // C s_26_10: cast zx s_26_9 -> bv
        let s_26_10: Bits = Bits::new(s_26_9 as u128, 64u16);
        // C s_26_11: lsl s_26_10 s_26_8
        let s_26_11: Bits = s_26_10 << s_26_8;
        // C s_26_12: sub s_26_11 s_26_10
        let s_26_12: Bits = ((s_26_11) - (s_26_10));
        // S s_26_13: and s_26_7 s_26_12
        let s_26_13: Bits = ((s_26_7) & (s_26_12));
        // S s_26_14: lsl s_26_13 s_26_4
        let s_26_14: Bits = s_26_13 << s_26_4;
        // C s_26_15: lsl s_26_12 s_26_4
        let s_26_15: Bits = s_26_12 << s_26_4;
        // C s_26_16: cmpl s_26_15
        let s_26_16: Bits = !s_26_15;
        // D s_26_17: and s_26_6 s_26_16
        let s_26_17: Bits = ((s_26_6) & (s_26_16));
        // D s_26_18: or s_26_17 s_26_14
        let s_26_18: Bits = ((s_26_17) | (s_26_14));
        // D s_26_19: cast reint s_26_18 -> u20
        let s_26_19: u32 = (s_26_18.value() as u32);
        // D s_26_20: write-var iss <= s_26_19
        fn_state.iss = s_26_19;
        // N s_26_21: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_27_0: const #5s : i64
        let s_27_0: i64 = 5;
        // C s_27_1: cast zx s_27_0 -> i
        let s_27_1: i128 = (i128::try_from(s_27_0).unwrap());
        // S s_27_2: call __UNKNOWN_bits(s_27_1)
        let s_27_2: Bits = u__UNKNOWN_bits(state, tracer, s_27_1);
        // S s_27_3: cast reint s_27_2 -> u8
        let s_27_3: u8 = (s_27_2.value() as u8);
        // C s_27_4: const #10s : i
        let s_27_4: i128 = 10;
        // D s_27_5: read-var iss:u20
        let s_27_5: u32 = fn_state.iss;
        // D s_27_6: cast zx s_27_5 -> bv
        let s_27_6: Bits = Bits::new(s_27_5 as u128, 20u16);
        // S s_27_7: cast zx s_27_3 -> bv
        let s_27_7: Bits = Bits::new(s_27_3 as u128, 5u16);
        // C s_27_8: const #4s : i
        let s_27_8: i128 = 4;
        // C s_27_9: const #1u : u64
        let s_27_9: u64 = 1;
        // C s_27_10: cast zx s_27_9 -> bv
        let s_27_10: Bits = Bits::new(s_27_9 as u128, 64u16);
        // C s_27_11: lsl s_27_10 s_27_8
        let s_27_11: Bits = s_27_10 << s_27_8;
        // C s_27_12: sub s_27_11 s_27_10
        let s_27_12: Bits = ((s_27_11) - (s_27_10));
        // S s_27_13: and s_27_7 s_27_12
        let s_27_13: Bits = ((s_27_7) & (s_27_12));
        // S s_27_14: lsl s_27_13 s_27_4
        let s_27_14: Bits = s_27_13 << s_27_4;
        // C s_27_15: lsl s_27_12 s_27_4
        let s_27_15: Bits = s_27_12 << s_27_4;
        // C s_27_16: cmpl s_27_15
        let s_27_16: Bits = !s_27_15;
        // D s_27_17: and s_27_6 s_27_16
        let s_27_17: Bits = ((s_27_6) & (s_27_16));
        // D s_27_18: or s_27_17 s_27_14
        let s_27_18: Bits = ((s_27_17) | (s_27_14));
        // D s_27_19: cast reint s_27_18 -> u20
        let s_27_19: u32 = (s_27_18.value() as u32);
        // D s_27_20: write-var iss <= s_27_19
        fn_state.iss = s_27_19;
        // N s_27_21: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_28_0: const #1u : u8
        let s_28_0: bool = true;
        // D s_28_1: write-var gs#24936 <= s_28_0
        fn_state.gs_24936 = s_28_0;
        // N s_28_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_29_0: const #1u : u8
        let s_29_0: bool = true;
        // D s_29_1: write-var gs#24937 <= s_29_0
        fn_state.gs_24937 = s_29_0;
        // N s_29_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_30_0: read-var except.1:struct
        let s_30_0: u32 = fn_state.except._1;
        // C s_30_1: const #8u : u32
        let s_30_1: u32 = 8;
        // D s_30_2: cmp-eq s_30_0 s_30_1
        let s_30_2: bool = ((s_30_0) == (s_30_1));
        // N s_30_3: branch s_30_2 b43 b31
        if s_30_2 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_31(state, tracer, fn_state);
        };
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_31_0: const #17s : i
        let s_31_0: i128 = 17;
        // D s_31_1: read-var iss:u20
        let s_31_1: u32 = fn_state.iss;
        // D s_31_2: cast zx s_31_1 -> bv
        let s_31_2: Bits = Bits::new(s_31_1 as u128, 20u16);
        // C s_31_3: const #0u : u8
        let s_31_3: u8 = 0;
        // C s_31_4: cast zx s_31_3 -> bv
        let s_31_4: Bits = Bits::new(s_31_3 as u128, 3u16);
        // C s_31_5: const #2s : i
        let s_31_5: i128 = 2;
        // C s_31_6: const #1u : u64
        let s_31_6: u64 = 1;
        // C s_31_7: cast zx s_31_6 -> bv
        let s_31_7: Bits = Bits::new(s_31_6 as u128, 64u16);
        // C s_31_8: lsl s_31_7 s_31_5
        let s_31_8: Bits = s_31_7 << s_31_5;
        // C s_31_9: sub s_31_8 s_31_7
        let s_31_9: Bits = ((s_31_8) - (s_31_7));
        // C s_31_10: and s_31_4 s_31_9
        let s_31_10: Bits = ((s_31_4) & (s_31_9));
        // C s_31_11: lsl s_31_10 s_31_0
        let s_31_11: Bits = s_31_10 << s_31_0;
        // C s_31_12: lsl s_31_9 s_31_0
        let s_31_12: Bits = s_31_9 << s_31_0;
        // C s_31_13: cmpl s_31_12
        let s_31_13: Bits = !s_31_12;
        // D s_31_14: and s_31_2 s_31_13
        let s_31_14: Bits = ((s_31_2) & (s_31_13));
        // D s_31_15: or s_31_14 s_31_11
        let s_31_15: Bits = ((s_31_14) | (s_31_11));
        // D s_31_16: cast reint s_31_15 -> u20
        let s_31_16: u32 = (s_31_15.value() as u32);
        // D s_31_17: write-var iss <= s_31_16
        fn_state.iss = s_31_16;
        // C s_31_18: const #14s : i
        let s_31_18: i128 = 14;
        // D s_31_19: read-var iss:u20
        let s_31_19: u32 = fn_state.iss;
        // D s_31_20: cast zx s_31_19 -> bv
        let s_31_20: Bits = Bits::new(s_31_19 as u128, 20u16);
        // C s_31_21: const #7u : u8
        let s_31_21: u8 = 7;
        // C s_31_22: cast zx s_31_21 -> bv
        let s_31_22: Bits = Bits::new(s_31_21 as u128, 3u16);
        // C s_31_23: const #2s : i
        let s_31_23: i128 = 2;
        // C s_31_24: const #1u : u64
        let s_31_24: u64 = 1;
        // C s_31_25: cast zx s_31_24 -> bv
        let s_31_25: Bits = Bits::new(s_31_24 as u128, 64u16);
        // C s_31_26: lsl s_31_25 s_31_23
        let s_31_26: Bits = s_31_25 << s_31_23;
        // C s_31_27: sub s_31_26 s_31_25
        let s_31_27: Bits = ((s_31_26) - (s_31_25));
        // C s_31_28: and s_31_22 s_31_27
        let s_31_28: Bits = ((s_31_22) & (s_31_27));
        // C s_31_29: lsl s_31_28 s_31_18
        let s_31_29: Bits = s_31_28 << s_31_18;
        // C s_31_30: lsl s_31_27 s_31_18
        let s_31_30: Bits = s_31_27 << s_31_18;
        // C s_31_31: cmpl s_31_30
        let s_31_31: Bits = !s_31_30;
        // D s_31_32: and s_31_20 s_31_31
        let s_31_32: Bits = ((s_31_20) & (s_31_31));
        // D s_31_33: or s_31_32 s_31_29
        let s_31_33: Bits = ((s_31_32) | (s_31_29));
        // D s_31_34: cast reint s_31_33 -> u20
        let s_31_34: u32 = (s_31_33.value() as u32);
        // D s_31_35: write-var iss <= s_31_34
        fn_state.iss = s_31_34;
        // C s_31_36: const #16s : i
        let s_31_36: i128 = 16;
        // D s_31_37: read-var instr:u32
        let s_31_37: u32 = fn_state.instr;
        // D s_31_38: cast zx s_31_37 -> bv
        let s_31_38: Bits = Bits::new(s_31_37 as u128, 32u16);
        // C s_31_39: const #1s : i64
        let s_31_39: i64 = 1;
        // C s_31_40: cast zx s_31_39 -> i
        let s_31_40: i128 = (i128::try_from(s_31_39).unwrap());
        // C s_31_41: const #3s : i
        let s_31_41: i128 = 3;
        // C s_31_42: add s_31_41 s_31_40
        let s_31_42: i128 = (s_31_41 + s_31_40);
        // D s_31_43: bit-extract s_31_38 s_31_36 s_31_42
        let s_31_43: Bits = (Bits::new(
            ((s_31_38) >> (s_31_36)).value(),
            u16::try_from(s_31_42).unwrap(),
        ));
        // D s_31_44: cast reint s_31_43 -> u8
        let s_31_44: u8 = (s_31_43.value() as u8);
        // C s_31_45: const #10s : i
        let s_31_45: i128 = 10;
        // D s_31_46: read-var iss:u20
        let s_31_46: u32 = fn_state.iss;
        // D s_31_47: cast zx s_31_46 -> bv
        let s_31_47: Bits = Bits::new(s_31_46 as u128, 20u16);
        // D s_31_48: cast zx s_31_44 -> bv
        let s_31_48: Bits = Bits::new(s_31_44 as u128, 4u16);
        // C s_31_49: const #3s : i
        let s_31_49: i128 = 3;
        // C s_31_50: const #1u : u64
        let s_31_50: u64 = 1;
        // C s_31_51: cast zx s_31_50 -> bv
        let s_31_51: Bits = Bits::new(s_31_50 as u128, 64u16);
        // C s_31_52: lsl s_31_51 s_31_49
        let s_31_52: Bits = s_31_51 << s_31_49;
        // C s_31_53: sub s_31_52 s_31_51
        let s_31_53: Bits = ((s_31_52) - (s_31_51));
        // D s_31_54: and s_31_48 s_31_53
        let s_31_54: Bits = ((s_31_48) & (s_31_53));
        // D s_31_55: lsl s_31_54 s_31_45
        let s_31_55: Bits = s_31_54 << s_31_45;
        // C s_31_56: lsl s_31_53 s_31_45
        let s_31_56: Bits = s_31_53 << s_31_45;
        // C s_31_57: cmpl s_31_56
        let s_31_57: Bits = !s_31_56;
        // D s_31_58: and s_31_47 s_31_57
        let s_31_58: Bits = ((s_31_47) & (s_31_57));
        // D s_31_59: or s_31_58 s_31_55
        let s_31_59: Bits = ((s_31_58) | (s_31_55));
        // D s_31_60: cast reint s_31_59 -> u20
        let s_31_60: u32 = (s_31_59.value() as u32);
        // D s_31_61: write-var iss <= s_31_60
        fn_state.iss = s_31_60;
        // C s_31_62: const #1s : i
        let s_31_62: i128 = 1;
        // D s_31_63: read-var iss:u20
        let s_31_63: u32 = fn_state.iss;
        // D s_31_64: cast zx s_31_63 -> bv
        let s_31_64: Bits = Bits::new(s_31_63 as u128, 20u16);
        // C s_31_65: const #0u : u8
        let s_31_65: u8 = 0;
        // C s_31_66: cast zx s_31_65 -> bv
        let s_31_66: Bits = Bits::new(s_31_65 as u128, 4u16);
        // C s_31_67: const #3s : i
        let s_31_67: i128 = 3;
        // C s_31_68: const #1u : u64
        let s_31_68: u64 = 1;
        // C s_31_69: cast zx s_31_68 -> bv
        let s_31_69: Bits = Bits::new(s_31_68 as u128, 64u16);
        // C s_31_70: lsl s_31_69 s_31_67
        let s_31_70: Bits = s_31_69 << s_31_67;
        // C s_31_71: sub s_31_70 s_31_69
        let s_31_71: Bits = ((s_31_70) - (s_31_69));
        // C s_31_72: and s_31_66 s_31_71
        let s_31_72: Bits = ((s_31_66) & (s_31_71));
        // C s_31_73: lsl s_31_72 s_31_62
        let s_31_73: Bits = s_31_72 << s_31_62;
        // C s_31_74: lsl s_31_71 s_31_62
        let s_31_74: Bits = s_31_71 << s_31_62;
        // C s_31_75: cmpl s_31_74
        let s_31_75: Bits = !s_31_74;
        // D s_31_76: and s_31_64 s_31_75
        let s_31_76: Bits = ((s_31_64) & (s_31_75));
        // D s_31_77: or s_31_76 s_31_73
        let s_31_77: Bits = ((s_31_76) | (s_31_73));
        // D s_31_78: cast reint s_31_77 -> u20
        let s_31_78: u32 = (s_31_77.value() as u32);
        // D s_31_79: write-var iss <= s_31_78
        fn_state.iss = s_31_78;
        // N s_31_80: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_32_0: const #20s : i
        let s_32_0: i128 = 20;
        // D s_32_1: read-var instr:u32
        let s_32_1: u32 = fn_state.instr;
        // D s_32_2: cast zx s_32_1 -> bv
        let s_32_2: Bits = Bits::new(s_32_1 as u128, 32u16);
        // C s_32_3: const #1u : u64
        let s_32_3: u64 = 1;
        // D s_32_4: bit-extract s_32_2 s_32_0 s_32_3
        let s_32_4: Bits = (Bits::new(
            ((s_32_2) >> (s_32_0)).value(),
            u16::try_from(s_32_3).unwrap(),
        ));
        // D s_32_5: cast reint s_32_4 -> u8
        let s_32_5: bool = ((s_32_4.value()) != 0);
        // C s_32_6: const #0s : i
        let s_32_6: i128 = 0;
        // C s_32_7: const #0u : u64
        let s_32_7: u64 = 0;
        // D s_32_8: cast zx s_32_5 -> u64
        let s_32_8: u64 = (s_32_5 as u64);
        // C s_32_9: const #1u : u64
        let s_32_9: u64 = 1;
        // D s_32_10: and s_32_8 s_32_9
        let s_32_10: u64 = ((s_32_8) & (s_32_9));
        // D s_32_11: cmp-eq s_32_10 s_32_9
        let s_32_11: bool = ((s_32_10) == (s_32_9));
        // D s_32_12: lsl s_32_8 s_32_6
        let s_32_12: u64 = s_32_8 << s_32_6;
        // D s_32_13: or s_32_7 s_32_12
        let s_32_13: u64 = ((s_32_7) | (s_32_12));
        // D s_32_14: cmpl s_32_12
        let s_32_14: u64 = !s_32_12;
        // D s_32_15: and s_32_7 s_32_14
        let s_32_15: u64 = ((s_32_7) & (s_32_14));
        // D s_32_16: select s_32_11 s_32_13 s_32_15
        let s_32_16: u64 = if s_32_11 { s_32_13 } else { s_32_15 };
        // D s_32_17: cast trunc s_32_16 -> u8
        let s_32_17: bool = ((s_32_16) != 0);
        // D s_32_18: cast zx s_32_17 -> bv
        let s_32_18: Bits = Bits::new(s_32_17 as u128, 1u16);
        // C s_32_19: const #1u : u8
        let s_32_19: bool = true;
        // C s_32_20: cast zx s_32_19 -> bv
        let s_32_20: Bits = Bits::new(s_32_19 as u128, 1u16);
        // D s_32_21: cmp-eq s_32_18 s_32_20
        let s_32_21: bool = ((s_32_18) == (s_32_20));
        // N s_32_22: branch s_32_21 b42 b33
        if s_32_21 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_33_0: const #0u : u8
        let s_33_0: bool = false;
        // D s_33_1: write-var gs#25032 <= s_33_0
        fn_state.gs_25032 = s_33_0;
        // N s_33_2: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_34_0: read-var gs#25032:u8
        let s_34_0: bool = fn_state.gs_25032;
        // N s_34_1: branch s_34_0 b41 b35
        if s_34_0 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_35_0: const #20s : i
        let s_35_0: i128 = 20;
        // D s_35_1: read-var instr:u32
        let s_35_1: u32 = fn_state.instr;
        // D s_35_2: cast zx s_35_1 -> bv
        let s_35_2: Bits = Bits::new(s_35_1 as u128, 32u16);
        // C s_35_3: const #1u : u64
        let s_35_3: u64 = 1;
        // D s_35_4: bit-extract s_35_2 s_35_0 s_35_3
        let s_35_4: Bits = (Bits::new(
            ((s_35_2) >> (s_35_0)).value(),
            u16::try_from(s_35_3).unwrap(),
        ));
        // D s_35_5: cast reint s_35_4 -> u8
        let s_35_5: bool = ((s_35_4.value()) != 0);
        // C s_35_6: const #0s : i
        let s_35_6: i128 = 0;
        // C s_35_7: const #0u : u64
        let s_35_7: u64 = 0;
        // D s_35_8: cast zx s_35_5 -> u64
        let s_35_8: u64 = (s_35_5 as u64);
        // C s_35_9: const #1u : u64
        let s_35_9: u64 = 1;
        // D s_35_10: and s_35_8 s_35_9
        let s_35_10: u64 = ((s_35_8) & (s_35_9));
        // D s_35_11: cmp-eq s_35_10 s_35_9
        let s_35_11: bool = ((s_35_10) == (s_35_9));
        // D s_35_12: lsl s_35_8 s_35_6
        let s_35_12: u64 = s_35_8 << s_35_6;
        // D s_35_13: or s_35_7 s_35_12
        let s_35_13: u64 = ((s_35_7) | (s_35_12));
        // D s_35_14: cmpl s_35_12
        let s_35_14: u64 = !s_35_12;
        // D s_35_15: and s_35_7 s_35_14
        let s_35_15: u64 = ((s_35_7) & (s_35_14));
        // D s_35_16: select s_35_11 s_35_13 s_35_15
        let s_35_16: u64 = if s_35_11 { s_35_13 } else { s_35_15 };
        // D s_35_17: cast trunc s_35_16 -> u8
        let s_35_17: bool = ((s_35_16) != 0);
        // D s_35_18: cast zx s_35_17 -> bv
        let s_35_18: Bits = Bits::new(s_35_17 as u128, 1u16);
        // C s_35_19: const #0u : u8
        let s_35_19: bool = false;
        // C s_35_20: cast zx s_35_19 -> bv
        let s_35_20: Bits = Bits::new(s_35_19 as u128, 1u16);
        // D s_35_21: cmp-eq s_35_18 s_35_20
        let s_35_21: bool = ((s_35_18) == (s_35_20));
        // N s_35_22: branch s_35_21 b40 b36
        if s_35_21 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#25037 <= s_36_0
        fn_state.gs_25037 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_37_0: read-var gs#25037:u8
        let s_37_0: bool = fn_state.gs_25037;
        // N s_37_1: branch s_37_0 b39 b38
        if s_37_0 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_38_0: const #12s : i
        let s_38_0: i128 = 12;
        // D s_38_1: read-var instr:u32
        let s_38_1: u32 = fn_state.instr;
        // D s_38_2: cast zx s_38_1 -> bv
        let s_38_2: Bits = Bits::new(s_38_1 as u128, 32u16);
        // C s_38_3: const #1s : i64
        let s_38_3: i64 = 1;
        // C s_38_4: cast zx s_38_3 -> i
        let s_38_4: i128 = (i128::try_from(s_38_3).unwrap());
        // C s_38_5: const #3s : i
        let s_38_5: i128 = 3;
        // C s_38_6: add s_38_5 s_38_4
        let s_38_6: i128 = (s_38_5 + s_38_4);
        // D s_38_7: bit-extract s_38_2 s_38_0 s_38_6
        let s_38_7: Bits = (Bits::new(
            ((s_38_2) >> (s_38_0)).value(),
            u16::try_from(s_38_6).unwrap(),
        ));
        // D s_38_8: cast reint s_38_7 -> u8
        let s_38_8: u8 = (s_38_7.value() as u8);
        // D s_38_9: cast zx s_38_8 -> bv
        let s_38_9: Bits = Bits::new(s_38_8 as u128, 4u16);
        // D s_38_10: cast zx s_38_9 -> i
        let s_38_10: i128 = (s_38_9.value() as i128);
        // D s_38_11: cast reint s_38_10 -> i64
        let s_38_11: i64 = (s_38_10 as i64);
        // C s_38_12: const #16983u : u32
        let s_38_12: u32 = 16983;
        // D s_38_13: read-reg s_38_12:u8
        let s_38_13: u8 = {
            let value = state.read_register::<u8>(s_38_12 as isize);
            tracer.read_register(s_38_12 as isize, value);
            value
        };
        // D s_38_14: call LookUpRIndex(s_38_11, s_38_13)
        let s_38_14: i64 = LookUpRIndex(state, tracer, s_38_11, s_38_13);
        // C s_38_15: const #4s : i
        let s_38_15: i128 = 4;
        // C s_38_16: const #0s : i
        let s_38_16: i128 = 0;
        // D s_38_17: cast zx s_38_14 -> i
        let s_38_17: i128 = (i128::try_from(s_38_14).unwrap());
        // D s_38_18: call integer_subrange(s_38_17, s_38_15, s_38_16)
        let s_38_18: Bits = integer_subrange(state, tracer, s_38_17, s_38_15, s_38_16);
        // D s_38_19: cast reint s_38_18 -> u8
        let s_38_19: u8 = (s_38_18.value() as u8);
        // C s_38_20: const #5s : i
        let s_38_20: i128 = 5;
        // D s_38_21: read-var iss:u20
        let s_38_21: u32 = fn_state.iss;
        // D s_38_22: cast zx s_38_21 -> bv
        let s_38_22: Bits = Bits::new(s_38_21 as u128, 20u16);
        // D s_38_23: cast zx s_38_19 -> bv
        let s_38_23: Bits = Bits::new(s_38_19 as u128, 5u16);
        // C s_38_24: const #4s : i
        let s_38_24: i128 = 4;
        // C s_38_25: const #1u : u64
        let s_38_25: u64 = 1;
        // C s_38_26: cast zx s_38_25 -> bv
        let s_38_26: Bits = Bits::new(s_38_25 as u128, 64u16);
        // C s_38_27: lsl s_38_26 s_38_24
        let s_38_27: Bits = s_38_26 << s_38_24;
        // C s_38_28: sub s_38_27 s_38_26
        let s_38_28: Bits = ((s_38_27) - (s_38_26));
        // D s_38_29: and s_38_23 s_38_28
        let s_38_29: Bits = ((s_38_23) & (s_38_28));
        // D s_38_30: lsl s_38_29 s_38_20
        let s_38_30: Bits = s_38_29 << s_38_20;
        // C s_38_31: lsl s_38_28 s_38_20
        let s_38_31: Bits = s_38_28 << s_38_20;
        // C s_38_32: cmpl s_38_31
        let s_38_32: Bits = !s_38_31;
        // D s_38_33: and s_38_22 s_38_32
        let s_38_33: Bits = ((s_38_22) & (s_38_32));
        // D s_38_34: or s_38_33 s_38_30
        let s_38_34: Bits = ((s_38_33) | (s_38_30));
        // D s_38_35: cast reint s_38_34 -> u20
        let s_38_35: u32 = (s_38_34.value() as u32);
        // D s_38_36: write-var iss <= s_38_35
        fn_state.iss = s_38_35;
        // N s_38_37: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_39_0: const #5s : i64
        let s_39_0: i64 = 5;
        // C s_39_1: cast zx s_39_0 -> i
        let s_39_1: i128 = (i128::try_from(s_39_0).unwrap());
        // S s_39_2: call __UNKNOWN_bits(s_39_1)
        let s_39_2: Bits = u__UNKNOWN_bits(state, tracer, s_39_1);
        // S s_39_3: cast reint s_39_2 -> u8
        let s_39_3: u8 = (s_39_2.value() as u8);
        // C s_39_4: const #5s : i
        let s_39_4: i128 = 5;
        // D s_39_5: read-var iss:u20
        let s_39_5: u32 = fn_state.iss;
        // D s_39_6: cast zx s_39_5 -> bv
        let s_39_6: Bits = Bits::new(s_39_5 as u128, 20u16);
        // S s_39_7: cast zx s_39_3 -> bv
        let s_39_7: Bits = Bits::new(s_39_3 as u128, 5u16);
        // C s_39_8: const #4s : i
        let s_39_8: i128 = 4;
        // C s_39_9: const #1u : u64
        let s_39_9: u64 = 1;
        // C s_39_10: cast zx s_39_9 -> bv
        let s_39_10: Bits = Bits::new(s_39_9 as u128, 64u16);
        // C s_39_11: lsl s_39_10 s_39_8
        let s_39_11: Bits = s_39_10 << s_39_8;
        // C s_39_12: sub s_39_11 s_39_10
        let s_39_12: Bits = ((s_39_11) - (s_39_10));
        // S s_39_13: and s_39_7 s_39_12
        let s_39_13: Bits = ((s_39_7) & (s_39_12));
        // S s_39_14: lsl s_39_13 s_39_4
        let s_39_14: Bits = s_39_13 << s_39_4;
        // C s_39_15: lsl s_39_12 s_39_4
        let s_39_15: Bits = s_39_12 << s_39_4;
        // C s_39_16: cmpl s_39_15
        let s_39_16: Bits = !s_39_15;
        // D s_39_17: and s_39_6 s_39_16
        let s_39_17: Bits = ((s_39_6) & (s_39_16));
        // D s_39_18: or s_39_17 s_39_14
        let s_39_18: Bits = ((s_39_17) | (s_39_14));
        // D s_39_19: cast reint s_39_18 -> u20
        let s_39_19: u32 = (s_39_18.value() as u32);
        // D s_39_20: write-var iss <= s_39_19
        fn_state.iss = s_39_19;
        // N s_39_21: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_40_0: const #12s : i
        let s_40_0: i128 = 12;
        // D s_40_1: read-var instr:u32
        let s_40_1: u32 = fn_state.instr;
        // D s_40_2: cast zx s_40_1 -> bv
        let s_40_2: Bits = Bits::new(s_40_1 as u128, 32u16);
        // C s_40_3: const #1s : i64
        let s_40_3: i64 = 1;
        // C s_40_4: cast zx s_40_3 -> i
        let s_40_4: i128 = (i128::try_from(s_40_3).unwrap());
        // C s_40_5: const #3s : i
        let s_40_5: i128 = 3;
        // C s_40_6: add s_40_5 s_40_4
        let s_40_6: i128 = (s_40_5 + s_40_4);
        // D s_40_7: bit-extract s_40_2 s_40_0 s_40_6
        let s_40_7: Bits = (Bits::new(
            ((s_40_2) >> (s_40_0)).value(),
            u16::try_from(s_40_6).unwrap(),
        ));
        // D s_40_8: cast reint s_40_7 -> u8
        let s_40_8: u8 = (s_40_7.value() as u8);
        // D s_40_9: cast zx s_40_8 -> bv
        let s_40_9: Bits = Bits::new(s_40_8 as u128, 4u16);
        // C s_40_10: const #15u : u8
        let s_40_10: u8 = 15;
        // C s_40_11: cast zx s_40_10 -> bv
        let s_40_11: Bits = Bits::new(s_40_10 as u128, 4u16);
        // D s_40_12: cmp-eq s_40_9 s_40_11
        let s_40_12: bool = ((s_40_9) == (s_40_11));
        // D s_40_13: write-var gs#25037 <= s_40_12
        fn_state.gs_25037 = s_40_12;
        // N s_40_14: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_41_0: const #5s : i
        let s_41_0: i128 = 5;
        // D s_41_1: read-var iss:u20
        let s_41_1: u32 = fn_state.iss;
        // D s_41_2: cast zx s_41_1 -> bv
        let s_41_2: Bits = Bits::new(s_41_1 as u128, 20u16);
        // C s_41_3: const #31u : u8
        let s_41_3: u8 = 31;
        // C s_41_4: cast zx s_41_3 -> bv
        let s_41_4: Bits = Bits::new(s_41_3 as u128, 5u16);
        // C s_41_5: const #4s : i
        let s_41_5: i128 = 4;
        // C s_41_6: const #1u : u64
        let s_41_6: u64 = 1;
        // C s_41_7: cast zx s_41_6 -> bv
        let s_41_7: Bits = Bits::new(s_41_6 as u128, 64u16);
        // C s_41_8: lsl s_41_7 s_41_5
        let s_41_8: Bits = s_41_7 << s_41_5;
        // C s_41_9: sub s_41_8 s_41_7
        let s_41_9: Bits = ((s_41_8) - (s_41_7));
        // C s_41_10: and s_41_4 s_41_9
        let s_41_10: Bits = ((s_41_4) & (s_41_9));
        // C s_41_11: lsl s_41_10 s_41_0
        let s_41_11: Bits = s_41_10 << s_41_0;
        // C s_41_12: lsl s_41_9 s_41_0
        let s_41_12: Bits = s_41_9 << s_41_0;
        // C s_41_13: cmpl s_41_12
        let s_41_13: Bits = !s_41_12;
        // D s_41_14: and s_41_2 s_41_13
        let s_41_14: Bits = ((s_41_2) & (s_41_13));
        // D s_41_15: or s_41_14 s_41_11
        let s_41_15: Bits = ((s_41_14) | (s_41_11));
        // D s_41_16: cast reint s_41_15 -> u20
        let s_41_16: u32 = (s_41_15.value() as u32);
        // D s_41_17: write-var iss <= s_41_16
        fn_state.iss = s_41_16;
        // N s_41_18: jump b15
        return block_15(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_42_0: const #12s : i
        let s_42_0: i128 = 12;
        // D s_42_1: read-var instr:u32
        let s_42_1: u32 = fn_state.instr;
        // D s_42_2: cast zx s_42_1 -> bv
        let s_42_2: Bits = Bits::new(s_42_1 as u128, 32u16);
        // C s_42_3: const #1s : i64
        let s_42_3: i64 = 1;
        // C s_42_4: cast zx s_42_3 -> i
        let s_42_4: i128 = (i128::try_from(s_42_3).unwrap());
        // C s_42_5: const #3s : i
        let s_42_5: i128 = 3;
        // C s_42_6: add s_42_5 s_42_4
        let s_42_6: i128 = (s_42_5 + s_42_4);
        // D s_42_7: bit-extract s_42_2 s_42_0 s_42_6
        let s_42_7: Bits = (Bits::new(
            ((s_42_2) >> (s_42_0)).value(),
            u16::try_from(s_42_6).unwrap(),
        ));
        // D s_42_8: cast reint s_42_7 -> u8
        let s_42_8: u8 = (s_42_7.value() as u8);
        // D s_42_9: cast zx s_42_8 -> bv
        let s_42_9: Bits = Bits::new(s_42_8 as u128, 4u16);
        // C s_42_10: const #15u : u8
        let s_42_10: u8 = 15;
        // C s_42_11: cast zx s_42_10 -> bv
        let s_42_11: Bits = Bits::new(s_42_10 as u128, 4u16);
        // D s_42_12: cmp-eq s_42_9 s_42_11
        let s_42_12: bool = ((s_42_9) == (s_42_11));
        // D s_42_13: write-var gs#25032 <= s_42_12
        fn_state.gs_25032 = s_42_12;
        // N s_42_14: jump b34
        return block_34(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_43_0: const #5s : i
        let s_43_0: i128 = 5;
        // D s_43_1: read-var instr:u32
        let s_43_1: u32 = fn_state.instr;
        // D s_43_2: cast zx s_43_1 -> bv
        let s_43_2: Bits = Bits::new(s_43_1 as u128, 32u16);
        // C s_43_3: const #1s : i64
        let s_43_3: i64 = 1;
        // C s_43_4: cast zx s_43_3 -> i
        let s_43_4: i128 = (i128::try_from(s_43_3).unwrap());
        // C s_43_5: const #2s : i
        let s_43_5: i128 = 2;
        // C s_43_6: add s_43_5 s_43_4
        let s_43_6: i128 = (s_43_5 + s_43_4);
        // D s_43_7: bit-extract s_43_2 s_43_0 s_43_6
        let s_43_7: Bits = (Bits::new(
            ((s_43_2) >> (s_43_0)).value(),
            u16::try_from(s_43_6).unwrap(),
        ));
        // D s_43_8: cast reint s_43_7 -> u8
        let s_43_8: u8 = (s_43_7.value() as u8);
        // C s_43_9: const #17s : i
        let s_43_9: i128 = 17;
        // D s_43_10: read-var iss:u20
        let s_43_10: u32 = fn_state.iss;
        // D s_43_11: cast zx s_43_10 -> bv
        let s_43_11: Bits = Bits::new(s_43_10 as u128, 20u16);
        // D s_43_12: cast zx s_43_8 -> bv
        let s_43_12: Bits = Bits::new(s_43_8 as u128, 3u16);
        // C s_43_13: const #2s : i
        let s_43_13: i128 = 2;
        // C s_43_14: const #1u : u64
        let s_43_14: u64 = 1;
        // C s_43_15: cast zx s_43_14 -> bv
        let s_43_15: Bits = Bits::new(s_43_14 as u128, 64u16);
        // C s_43_16: lsl s_43_15 s_43_13
        let s_43_16: Bits = s_43_15 << s_43_13;
        // C s_43_17: sub s_43_16 s_43_15
        let s_43_17: Bits = ((s_43_16) - (s_43_15));
        // D s_43_18: and s_43_12 s_43_17
        let s_43_18: Bits = ((s_43_12) & (s_43_17));
        // D s_43_19: lsl s_43_18 s_43_9
        let s_43_19: Bits = s_43_18 << s_43_9;
        // C s_43_20: lsl s_43_17 s_43_9
        let s_43_20: Bits = s_43_17 << s_43_9;
        // C s_43_21: cmpl s_43_20
        let s_43_21: Bits = !s_43_20;
        // D s_43_22: and s_43_11 s_43_21
        let s_43_22: Bits = ((s_43_11) & (s_43_21));
        // D s_43_23: or s_43_22 s_43_19
        let s_43_23: Bits = ((s_43_22) | (s_43_19));
        // D s_43_24: cast reint s_43_23 -> u20
        let s_43_24: u32 = (s_43_23.value() as u32);
        // D s_43_25: write-var iss <= s_43_24
        fn_state.iss = s_43_24;
        // C s_43_26: const #21s : i
        let s_43_26: i128 = 21;
        // D s_43_27: read-var instr:u32
        let s_43_27: u32 = fn_state.instr;
        // D s_43_28: cast zx s_43_27 -> bv
        let s_43_28: Bits = Bits::new(s_43_27 as u128, 32u16);
        // C s_43_29: const #1s : i64
        let s_43_29: i64 = 1;
        // C s_43_30: cast zx s_43_29 -> i
        let s_43_30: i128 = (i128::try_from(s_43_29).unwrap());
        // C s_43_31: const #2s : i
        let s_43_31: i128 = 2;
        // C s_43_32: add s_43_31 s_43_30
        let s_43_32: i128 = (s_43_31 + s_43_30);
        // D s_43_33: bit-extract s_43_28 s_43_26 s_43_32
        let s_43_33: Bits = (Bits::new(
            ((s_43_28) >> (s_43_26)).value(),
            u16::try_from(s_43_32).unwrap(),
        ));
        // D s_43_34: cast reint s_43_33 -> u8
        let s_43_34: u8 = (s_43_33.value() as u8);
        // C s_43_35: const #14s : i
        let s_43_35: i128 = 14;
        // D s_43_36: read-var iss:u20
        let s_43_36: u32 = fn_state.iss;
        // D s_43_37: cast zx s_43_36 -> bv
        let s_43_37: Bits = Bits::new(s_43_36 as u128, 20u16);
        // D s_43_38: cast zx s_43_34 -> bv
        let s_43_38: Bits = Bits::new(s_43_34 as u128, 3u16);
        // C s_43_39: const #2s : i
        let s_43_39: i128 = 2;
        // C s_43_40: const #1u : u64
        let s_43_40: u64 = 1;
        // C s_43_41: cast zx s_43_40 -> bv
        let s_43_41: Bits = Bits::new(s_43_40 as u128, 64u16);
        // C s_43_42: lsl s_43_41 s_43_39
        let s_43_42: Bits = s_43_41 << s_43_39;
        // C s_43_43: sub s_43_42 s_43_41
        let s_43_43: Bits = ((s_43_42) - (s_43_41));
        // D s_43_44: and s_43_38 s_43_43
        let s_43_44: Bits = ((s_43_38) & (s_43_43));
        // D s_43_45: lsl s_43_44 s_43_35
        let s_43_45: Bits = s_43_44 << s_43_35;
        // C s_43_46: lsl s_43_43 s_43_35
        let s_43_46: Bits = s_43_43 << s_43_35;
        // C s_43_47: cmpl s_43_46
        let s_43_47: Bits = !s_43_46;
        // D s_43_48: and s_43_37 s_43_47
        let s_43_48: Bits = ((s_43_37) & (s_43_47));
        // D s_43_49: or s_43_48 s_43_45
        let s_43_49: Bits = ((s_43_48) | (s_43_45));
        // D s_43_50: cast reint s_43_49 -> u20
        let s_43_50: u32 = (s_43_49.value() as u32);
        // D s_43_51: write-var iss <= s_43_50
        fn_state.iss = s_43_50;
        // C s_43_52: const #16s : i
        let s_43_52: i128 = 16;
        // D s_43_53: read-var instr:u32
        let s_43_53: u32 = fn_state.instr;
        // D s_43_54: cast zx s_43_53 -> bv
        let s_43_54: Bits = Bits::new(s_43_53 as u128, 32u16);
        // C s_43_55: const #1s : i64
        let s_43_55: i64 = 1;
        // C s_43_56: cast zx s_43_55 -> i
        let s_43_56: i128 = (i128::try_from(s_43_55).unwrap());
        // C s_43_57: const #3s : i
        let s_43_57: i128 = 3;
        // C s_43_58: add s_43_57 s_43_56
        let s_43_58: i128 = (s_43_57 + s_43_56);
        // D s_43_59: bit-extract s_43_54 s_43_52 s_43_58
        let s_43_59: Bits = (Bits::new(
            ((s_43_54) >> (s_43_52)).value(),
            u16::try_from(s_43_58).unwrap(),
        ));
        // D s_43_60: cast reint s_43_59 -> u8
        let s_43_60: u8 = (s_43_59.value() as u8);
        // C s_43_61: const #10s : i
        let s_43_61: i128 = 10;
        // D s_43_62: read-var iss:u20
        let s_43_62: u32 = fn_state.iss;
        // D s_43_63: cast zx s_43_62 -> bv
        let s_43_63: Bits = Bits::new(s_43_62 as u128, 20u16);
        // D s_43_64: cast zx s_43_60 -> bv
        let s_43_64: Bits = Bits::new(s_43_60 as u128, 4u16);
        // C s_43_65: const #3s : i
        let s_43_65: i128 = 3;
        // C s_43_66: const #1u : u64
        let s_43_66: u64 = 1;
        // C s_43_67: cast zx s_43_66 -> bv
        let s_43_67: Bits = Bits::new(s_43_66 as u128, 64u16);
        // C s_43_68: lsl s_43_67 s_43_65
        let s_43_68: Bits = s_43_67 << s_43_65;
        // C s_43_69: sub s_43_68 s_43_67
        let s_43_69: Bits = ((s_43_68) - (s_43_67));
        // D s_43_70: and s_43_64 s_43_69
        let s_43_70: Bits = ((s_43_64) & (s_43_69));
        // D s_43_71: lsl s_43_70 s_43_61
        let s_43_71: Bits = s_43_70 << s_43_61;
        // C s_43_72: lsl s_43_69 s_43_61
        let s_43_72: Bits = s_43_69 << s_43_61;
        // C s_43_73: cmpl s_43_72
        let s_43_73: Bits = !s_43_72;
        // D s_43_74: and s_43_63 s_43_73
        let s_43_74: Bits = ((s_43_63) & (s_43_73));
        // D s_43_75: or s_43_74 s_43_71
        let s_43_75: Bits = ((s_43_74) | (s_43_71));
        // D s_43_76: cast reint s_43_75 -> u20
        let s_43_76: u32 = (s_43_75.value() as u32);
        // D s_43_77: write-var iss <= s_43_76
        fn_state.iss = s_43_76;
        // C s_43_78: const #0s : i
        let s_43_78: i128 = 0;
        // D s_43_79: read-var instr:u32
        let s_43_79: u32 = fn_state.instr;
        // D s_43_80: cast zx s_43_79 -> bv
        let s_43_80: Bits = Bits::new(s_43_79 as u128, 32u16);
        // C s_43_81: const #1s : i64
        let s_43_81: i64 = 1;
        // C s_43_82: cast zx s_43_81 -> i
        let s_43_82: i128 = (i128::try_from(s_43_81).unwrap());
        // C s_43_83: const #3s : i
        let s_43_83: i128 = 3;
        // C s_43_84: add s_43_83 s_43_82
        let s_43_84: i128 = (s_43_83 + s_43_82);
        // D s_43_85: bit-extract s_43_80 s_43_78 s_43_84
        let s_43_85: Bits = (Bits::new(
            ((s_43_80) >> (s_43_78)).value(),
            u16::try_from(s_43_84).unwrap(),
        ));
        // D s_43_86: cast reint s_43_85 -> u8
        let s_43_86: u8 = (s_43_85.value() as u8);
        // C s_43_87: const #1s : i
        let s_43_87: i128 = 1;
        // D s_43_88: read-var iss:u20
        let s_43_88: u32 = fn_state.iss;
        // D s_43_89: cast zx s_43_88 -> bv
        let s_43_89: Bits = Bits::new(s_43_88 as u128, 20u16);
        // D s_43_90: cast zx s_43_86 -> bv
        let s_43_90: Bits = Bits::new(s_43_86 as u128, 4u16);
        // C s_43_91: const #3s : i
        let s_43_91: i128 = 3;
        // C s_43_92: const #1u : u64
        let s_43_92: u64 = 1;
        // C s_43_93: cast zx s_43_92 -> bv
        let s_43_93: Bits = Bits::new(s_43_92 as u128, 64u16);
        // C s_43_94: lsl s_43_93 s_43_91
        let s_43_94: Bits = s_43_93 << s_43_91;
        // C s_43_95: sub s_43_94 s_43_93
        let s_43_95: Bits = ((s_43_94) - (s_43_93));
        // D s_43_96: and s_43_90 s_43_95
        let s_43_96: Bits = ((s_43_90) & (s_43_95));
        // D s_43_97: lsl s_43_96 s_43_87
        let s_43_97: Bits = s_43_96 << s_43_87;
        // C s_43_98: lsl s_43_95 s_43_87
        let s_43_98: Bits = s_43_95 << s_43_87;
        // C s_43_99: cmpl s_43_98
        let s_43_99: Bits = !s_43_98;
        // D s_43_100: and s_43_89 s_43_99
        let s_43_100: Bits = ((s_43_89) & (s_43_99));
        // D s_43_101: or s_43_100 s_43_97
        let s_43_101: Bits = ((s_43_100) | (s_43_97));
        // D s_43_102: cast reint s_43_101 -> u20
        let s_43_102: u32 = (s_43_101.value() as u32);
        // D s_43_103: write-var iss <= s_43_102
        fn_state.iss = s_43_102;
        // N s_43_104: jump b32
        return block_32(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_44_0: const #1u : u8
        let s_44_0: bool = true;
        // D s_44_1: write-var gs#24934 <= s_44_0
        fn_state.gs_24934 = s_44_0;
        // N s_44_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_45_0: const #1u : u8
        let s_45_0: bool = true;
        // D s_45_1: write-var gs#24935 <= s_45_0
        fn_state.gs_24935 = s_45_0;
        // N s_45_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_46_0: read-var except:struct
        let s_46_0: ProductTypeb7f99f96751e17c4 = fn_state.except;
        // D s_46_1: write-var return_value <= s_46_0
        fn_state.return_value = s_46_0;
        // N s_46_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_47_0: read-var ec:i
        let s_47_0: i128 = fn_state.ec;
        // C s_47_1: const #3s : i
        let s_47_1: i128 = 3;
        // D s_47_2: cmp-eq s_47_0 s_47_1
        let s_47_2: bool = ((s_47_0) == (s_47_1));
        // D s_47_3: not s_47_2
        let s_47_3: bool = !s_47_2;
        // N s_47_4: branch s_47_3 b49 b48
        if s_47_3 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_48_0: const #2u : u32
        let s_48_0: u32 = 2;
        // S s_48_1: call ExceptionSyndrome(s_48_0)
        let s_48_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_48_0,
        );
        // D s_48_2: write-var except <= s_48_1
        fn_state.except = s_48_1;
        // N s_48_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_49_0: read-var ec:i
        let s_49_0: i128 = fn_state.ec;
        // C s_49_1: const #4s : i
        let s_49_1: i128 = 4;
        // D s_49_2: cmp-eq s_49_0 s_49_1
        let s_49_2: bool = ((s_49_0) == (s_49_1));
        // D s_49_3: not s_49_2
        let s_49_3: bool = !s_49_2;
        // N s_49_4: branch s_49_3 b51 b50
        if s_49_3 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_50_0: const #3u : u32
        let s_50_0: u32 = 3;
        // S s_50_1: call ExceptionSyndrome(s_50_0)
        let s_50_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_50_0,
        );
        // D s_50_2: write-var except <= s_50_1
        fn_state.except = s_50_1;
        // N s_50_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_51_0: read-var ec:i
        let s_51_0: i128 = fn_state.ec;
        // C s_51_1: const #5s : i
        let s_51_1: i128 = 5;
        // D s_51_2: cmp-eq s_51_0 s_51_1
        let s_51_2: bool = ((s_51_0) == (s_51_1));
        // D s_51_3: not s_51_2
        let s_51_3: bool = !s_51_2;
        // N s_51_4: branch s_51_3 b53 b52
        if s_51_3 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_52_0: const #4u : u32
        let s_52_0: u32 = 4;
        // S s_52_1: call ExceptionSyndrome(s_52_0)
        let s_52_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_52_0,
        );
        // D s_52_2: write-var except <= s_52_1
        fn_state.except = s_52_1;
        // N s_52_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_53_0: read-var ec:i
        let s_53_0: i128 = fn_state.ec;
        // C s_53_1: const #6s : i
        let s_53_1: i128 = 6;
        // D s_53_2: cmp-eq s_53_0 s_53_1
        let s_53_2: bool = ((s_53_0) == (s_53_1));
        // D s_53_3: not s_53_2
        let s_53_3: bool = !s_53_2;
        // N s_53_4: branch s_53_3 b55 b54
        if s_53_3 {
            return block_55(state, tracer, fn_state);
        } else {
            return block_54(state, tracer, fn_state);
        };
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_54_0: const #5u : u32
        let s_54_0: u32 = 5;
        // S s_54_1: call ExceptionSyndrome(s_54_0)
        let s_54_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_54_0,
        );
        // D s_54_2: write-var except <= s_54_1
        fn_state.except = s_54_1;
        // N s_54_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_55_0: read-var ec:i
        let s_55_0: i128 = fn_state.ec;
        // C s_55_1: const #7s : i
        let s_55_1: i128 = 7;
        // D s_55_2: cmp-eq s_55_0 s_55_1
        let s_55_2: bool = ((s_55_0) == (s_55_1));
        // D s_55_3: not s_55_2
        let s_55_3: bool = !s_55_2;
        // N s_55_4: branch s_55_3 b57 b56
        if s_55_3 {
            return block_57(state, tracer, fn_state);
        } else {
            return block_56(state, tracer, fn_state);
        };
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_56_0: const #7u : u32
        let s_56_0: u32 = 7;
        // S s_56_1: call ExceptionSyndrome(s_56_0)
        let s_56_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_56_0,
        );
        // D s_56_2: write-var except <= s_56_1
        fn_state.except = s_56_1;
        // N s_56_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_57_0: read-var ec:i
        let s_57_0: i128 = fn_state.ec;
        // C s_57_1: const #8s : i
        let s_57_1: i128 = 8;
        // D s_57_2: cmp-eq s_57_0 s_57_1
        let s_57_2: bool = ((s_57_0) == (s_57_1));
        // D s_57_3: not s_57_2
        let s_57_3: bool = !s_57_2;
        // N s_57_4: branch s_57_3 b59 b58
        if s_57_3 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_58(state, tracer, fn_state);
        };
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_58_0: const #8u : u32
        let s_58_0: u32 = 8;
        // S s_58_1: call ExceptionSyndrome(s_58_0)
        let s_58_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_58_0,
        );
        // D s_58_2: write-var except <= s_58_1
        fn_state.except = s_58_1;
        // N s_58_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // D s_59_0: read-var ec:i
        let s_59_0: i128 = fn_state.ec;
        // C s_59_1: const #12s : i
        let s_59_1: i128 = 12;
        // D s_59_2: cmp-eq s_59_0 s_59_1
        let s_59_2: bool = ((s_59_0) == (s_59_1));
        // D s_59_3: not s_59_2
        let s_59_3: bool = !s_59_2;
        // N s_59_4: branch s_59_3 b61 b60
        if s_59_3 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_60(state, tracer, fn_state);
        };
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_60_0: const #6u : u32
        let s_60_0: u32 = 6;
        // S s_60_1: call ExceptionSyndrome(s_60_0)
        let s_60_1: ProductTypeb7f99f96751e17c4 = ExceptionSyndrome(
            state,
            tracer,
            s_60_0,
        );
        // D s_60_2: write-var except <= s_60_1
        fn_state.except = s_60_1;
        // N s_60_3: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypeb7f99f96751e17c4 {
        // C s_61_0: const #() : ()
        let s_61_0: () = ();
        // S s_61_1: call Unreachable(s_61_0)
        let s_61_1: () = Unreachable(state, tracer, s_61_0);
        // N s_61_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
