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
use u_get_DBGWCR_Type_MASK::*;
use unsigned_subrange::*;
use IsZero::*;
use DBGWVR_read::*;
use u__id::*;
use DBGWCR_read::*;
use ConstrainUnpredictableBool::*;
use ConstrainUnpredictableInteger::*;
use subrange_subrange_eq::*;
use u_get_DBGWCR_Type_BAS::*;
use is_zero_subrange::*;
use IsOnes::*;
use common::*;
pub fn AArch32_WatchpointByteMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    vaddress: u32,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_30141: bool,
        bottomshadow_553: i64,
        gs_30158: bool,
        maskshadow_554: i128,
        ga_23334: ProductType396b95aa74979079,
        c: u32,
        ga_23364: ProductType700c18a878c5601b,
        ga_23348: ProductType700c18a878c5601b,
        gs_30171: bool,
        gs_30174: bool,
        mask: i128,
        byte_select_match: bool,
        ga_23346: ProductType700c18a878c5601b,
        ga_23353: ProductType700c18a878c5601b,
        return_value: bool,
        bottom: i64,
        WVR_match: bool,
        gs_30151: bool,
        gs_30159: bool,
        gs_30172: bool,
        gs_30178: bool,
        n: i64,
        vaddress: u32,
    }
    let fn_state = FunctionState {
        n,
        vaddress,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_0_0: read-var n:i64
        let s_0_0: i64 = fn_state.n;
        // D s_0_1: call DBGWVR_read(s_0_0)
        let s_0_1: ProductType700c18a878c5601b = DBGWVR_read(state, tracer, s_0_0);
        // D s_0_2: write-var ga#23364 <= s_0_1
        fn_state.ga_23364 = s_0_1;
        // D s_0_3: read-var ga#23364.0:struct
        let s_0_3: u32 = fn_state.ga_23364._0;
        // C s_0_4: const #2s : i
        let s_0_4: i128 = 2;
        // D s_0_5: cast zx s_0_3 -> bv
        let s_0_5: Bits = Bits::new(s_0_3 as u128, 32u16);
        // C s_0_6: const #1u : u64
        let s_0_6: u64 = 1;
        // D s_0_7: bit-extract s_0_5 s_0_4 s_0_6
        let s_0_7: Bits = (Bits::new(
            ((s_0_5) >> (s_0_4)).value(),
            u16::try_from(s_0_6).unwrap(),
        ));
        // D s_0_8: cast reint s_0_7 -> u8
        let s_0_8: bool = ((s_0_7.value()) != 0);
        // C s_0_9: const #0s : i
        let s_0_9: i128 = 0;
        // C s_0_10: const #0u : u64
        let s_0_10: u64 = 0;
        // D s_0_11: cast zx s_0_8 -> u64
        let s_0_11: u64 = (s_0_8 as u64);
        // C s_0_12: const #1u : u64
        let s_0_12: u64 = 1;
        // D s_0_13: and s_0_11 s_0_12
        let s_0_13: u64 = ((s_0_11) & (s_0_12));
        // D s_0_14: cmp-eq s_0_13 s_0_12
        let s_0_14: bool = ((s_0_13) == (s_0_12));
        // D s_0_15: lsl s_0_11 s_0_9
        let s_0_15: u64 = s_0_11 << s_0_9;
        // D s_0_16: or s_0_10 s_0_15
        let s_0_16: u64 = ((s_0_10) | (s_0_15));
        // D s_0_17: cmpl s_0_15
        let s_0_17: u64 = !s_0_15;
        // D s_0_18: and s_0_10 s_0_17
        let s_0_18: u64 = ((s_0_10) & (s_0_17));
        // D s_0_19: select s_0_14 s_0_16 s_0_18
        let s_0_19: u64 = if s_0_14 { s_0_16 } else { s_0_18 };
        // D s_0_20: cast trunc s_0_19 -> u8
        let s_0_20: bool = ((s_0_19) != 0);
        // D s_0_21: cast zx s_0_20 -> bv
        let s_0_21: Bits = Bits::new(s_0_20 as u128, 1u16);
        // C s_0_22: const #1u : u8
        let s_0_22: bool = true;
        // C s_0_23: cast zx s_0_22 -> bv
        let s_0_23: Bits = Bits::new(s_0_22 as u128, 1u16);
        // D s_0_24: cmp-eq s_0_21 s_0_23
        let s_0_24: bool = ((s_0_21) == (s_0_23));
        // N s_0_25: branch s_0_24 b48 b1
        if s_0_24 {
            return block_48(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_1_0: const #3s : i64
        let s_1_0: i64 = 3;
        // D s_1_1: write-var bottom <= s_1_0
        fn_state.bottom = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_2_0: read-var bottom:i64
        let s_2_0: i64 = fn_state.bottom;
        // D s_2_1: read-var n:i64
        let s_2_1: i64 = fn_state.n;
        // D s_2_2: call DBGWCR_read(s_2_1)
        let s_2_2: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_2_1);
        // D s_2_3: call _get_DBGWCR_Type_BAS(s_2_2)
        let s_2_3: u8 = u_get_DBGWCR_Type_BAS(state, tracer, s_2_2);
        // C s_2_4: const #1s : i
        let s_2_4: i128 = 1;
        // D s_2_5: cast zx s_2_0 -> i
        let s_2_5: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_6: sub s_2_5 s_2_4
        let s_2_6: i128 = ((s_2_5) - (s_2_4));
        // D s_2_7: cast reint s_2_6 -> i64
        let s_2_7: i64 = (s_2_6 as i64);
        // C s_2_8: const #0s : i
        let s_2_8: i128 = 0;
        // D s_2_9: read-var vaddress:u32
        let s_2_9: u32 = fn_state.vaddress;
        // D s_2_10: cast zx s_2_9 -> bv
        let s_2_10: Bits = Bits::new(s_2_9 as u128, 32u16);
        // D s_2_11: cast zx s_2_7 -> i
        let s_2_11: i128 = (i128::try_from(s_2_7).unwrap());
        // D s_2_12: call unsigned_subrange(s_2_10, s_2_11, s_2_8)
        let s_2_12: i128 = unsigned_subrange(state, tracer, s_2_10, s_2_11, s_2_8);
        // D s_2_13: cast reint s_2_12 -> i64
        let s_2_13: i64 = (s_2_12 as i64);
        // D s_2_14: cast zx s_2_3 -> bv
        let s_2_14: Bits = Bits::new(s_2_3 as u128, 8u16);
        // D s_2_15: cast zx s_2_13 -> i
        let s_2_15: i128 = (i128::try_from(s_2_13).unwrap());
        // C s_2_16: const #1u : u64
        let s_2_16: u64 = 1;
        // D s_2_17: bit-extract s_2_14 s_2_15 s_2_16
        let s_2_17: Bits = (Bits::new(
            ((s_2_14) >> (s_2_15)).value(),
            u16::try_from(s_2_16).unwrap(),
        ));
        // D s_2_18: cast reint s_2_17 -> u8
        let s_2_18: bool = ((s_2_17.value()) != 0);
        // C s_2_19: const #0s : i
        let s_2_19: i128 = 0;
        // C s_2_20: const #0u : u64
        let s_2_20: u64 = 0;
        // D s_2_21: cast zx s_2_18 -> u64
        let s_2_21: u64 = (s_2_18 as u64);
        // C s_2_22: const #1u : u64
        let s_2_22: u64 = 1;
        // D s_2_23: and s_2_21 s_2_22
        let s_2_23: u64 = ((s_2_21) & (s_2_22));
        // D s_2_24: cmp-eq s_2_23 s_2_22
        let s_2_24: bool = ((s_2_23) == (s_2_22));
        // D s_2_25: lsl s_2_21 s_2_19
        let s_2_25: u64 = s_2_21 << s_2_19;
        // D s_2_26: or s_2_20 s_2_25
        let s_2_26: u64 = ((s_2_20) | (s_2_25));
        // D s_2_27: cmpl s_2_25
        let s_2_27: u64 = !s_2_25;
        // D s_2_28: and s_2_20 s_2_27
        let s_2_28: u64 = ((s_2_20) & (s_2_27));
        // D s_2_29: select s_2_24 s_2_26 s_2_28
        let s_2_29: u64 = if s_2_24 { s_2_26 } else { s_2_28 };
        // D s_2_30: cast trunc s_2_29 -> u8
        let s_2_30: bool = ((s_2_29) != 0);
        // D s_2_31: cast zx s_2_30 -> bv
        let s_2_31: Bits = Bits::new(s_2_30 as u128, 1u16);
        // C s_2_32: const #0u : u8
        let s_2_32: bool = false;
        // C s_2_33: cast zx s_2_32 -> bv
        let s_2_33: Bits = Bits::new(s_2_32 as u128, 1u16);
        // D s_2_34: cmp-ne s_2_31 s_2_33
        let s_2_34: bool = ((s_2_31) != (s_2_33));
        // D s_2_35: write-var byte_select_match <= s_2_34
        fn_state.byte_select_match = s_2_34;
        // D s_2_36: read-var n:i64
        let s_2_36: i64 = fn_state.n;
        // D s_2_37: call DBGWCR_read(s_2_36)
        let s_2_37: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_2_36);
        // D s_2_38: call _get_DBGWCR_Type_MASK(s_2_37)
        let s_2_38: u8 = u_get_DBGWCR_Type_MASK(state, tracer, s_2_37);
        // D s_2_39: cast zx s_2_38 -> bv
        let s_2_39: Bits = Bits::new(s_2_38 as u128, 5u16);
        // D s_2_40: cast zx s_2_39 -> i
        let s_2_40: i128 = (s_2_39.value() as i128);
        // D s_2_41: write-var mask <= s_2_40
        fn_state.mask = s_2_40;
        // C s_2_42: const #0s : i
        let s_2_42: i128 = 0;
        // D s_2_43: read-var mask:i
        let s_2_43: i128 = fn_state.mask;
        // D s_2_44: cmp-gt s_2_43 s_2_42
        let s_2_44: bool = ((s_2_43) > (s_2_42));
        // N s_2_45: branch s_2_44 b47 b3
        if s_2_44 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#30141 <= s_3_0
        fn_state.gs_30141 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#30141:u8
        let s_4_0: bool = fn_state.gs_30141;
        // N s_4_1: branch s_4_0 b46 b5
        if s_4_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_5_0: read-var n:i64
        let s_5_0: i64 = fn_state.n;
        // D s_5_1: call DBGWCR_read(s_5_0)
        let s_5_1: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_5_0);
        // D s_5_2: call _get_DBGWCR_Type_BAS(s_5_1)
        let s_5_2: u8 = u_get_DBGWCR_Type_BAS(state, tracer, s_5_1);
        // D s_5_3: read-var n:i64
        let s_5_3: i64 = fn_state.n;
        // D s_5_4: call DBGWCR_read(s_5_3)
        let s_5_4: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_5_3);
        // D s_5_5: call _get_DBGWCR_Type_BAS(s_5_4)
        let s_5_5: u8 = u_get_DBGWCR_Type_BAS(state, tracer, s_5_4);
        // C s_5_6: const #1s : i
        let s_5_6: i128 = 1;
        // D s_5_7: cast zx s_5_5 -> bv
        let s_5_7: Bits = Bits::new(s_5_5 as u128, 8u16);
        // C s_5_8: cast cvt s_5_6 -> bv
        let s_5_8: Bits = Bits::new(s_5_6 as u128, 128);
        // D s_5_9: sub s_5_7 s_5_8
        let s_5_9: Bits = ((s_5_7) - (s_5_8));
        // D s_5_10: cast reint s_5_9 -> u8
        let s_5_10: u8 = (s_5_9.value() as u8);
        // D s_5_11: cast zx s_5_10 -> bv
        let s_5_11: Bits = Bits::new(s_5_10 as u128, 8u16);
        // D s_5_12: not s_5_11
        let s_5_12: Bits = !s_5_11;
        // D s_5_13: cast reint s_5_12 -> u8
        let s_5_13: u8 = (s_5_12.value() as u8);
        // D s_5_14: cast zx s_5_2 -> bv
        let s_5_14: Bits = Bits::new(s_5_2 as u128, 8u16);
        // D s_5_15: cast zx s_5_13 -> bv
        let s_5_15: Bits = Bits::new(s_5_13 as u128, 8u16);
        // D s_5_16: and s_5_14 s_5_15
        let s_5_16: Bits = ((s_5_14) & (s_5_15));
        // D s_5_17: cast reint s_5_16 -> u8
        let s_5_17: u8 = (s_5_16.value() as u8);
        // D s_5_18: read-var n:i64
        let s_5_18: i64 = fn_state.n;
        // D s_5_19: call DBGWCR_read(s_5_18)
        let s_5_19: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_5_18);
        // D s_5_20: call _get_DBGWCR_Type_BAS(s_5_19)
        let s_5_20: u8 = u_get_DBGWCR_Type_BAS(state, tracer, s_5_19);
        // D s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 8u16);
        // D s_5_22: cast zx s_5_17 -> bv
        let s_5_22: Bits = Bits::new(s_5_17 as u128, 8u16);
        // D s_5_23: add s_5_21 s_5_22
        let s_5_23: Bits = (s_5_21 + s_5_22);
        // D s_5_24: cast reint s_5_23 -> u8
        let s_5_24: u8 = (s_5_23.value() as u8);
        // C s_5_25: const #1s : i
        let s_5_25: i128 = 1;
        // D s_5_26: cast zx s_5_24 -> bv
        let s_5_26: Bits = Bits::new(s_5_24 as u128, 8u16);
        // C s_5_27: cast cvt s_5_25 -> bv
        let s_5_27: Bits = Bits::new(s_5_25 as u128, 128);
        // D s_5_28: sub s_5_26 s_5_27
        let s_5_28: Bits = ((s_5_26) - (s_5_27));
        // D s_5_29: cast reint s_5_28 -> u8
        let s_5_29: u8 = (s_5_28.value() as u8);
        // D s_5_30: cast zx s_5_24 -> bv
        let s_5_30: Bits = Bits::new(s_5_24 as u128, 8u16);
        // D s_5_31: cast zx s_5_29 -> bv
        let s_5_31: Bits = Bits::new(s_5_29 as u128, 8u16);
        // D s_5_32: and s_5_30 s_5_31
        let s_5_32: Bits = ((s_5_30) & (s_5_31));
        // D s_5_33: cast reint s_5_32 -> u8
        let s_5_33: u8 = (s_5_32.value() as u8);
        // D s_5_34: cast zx s_5_33 -> bv
        let s_5_34: Bits = Bits::new(s_5_33 as u128, 8u16);
        // D s_5_35: call IsZero(s_5_34)
        let s_5_35: bool = IsZero(state, tracer, s_5_34);
        // D s_5_36: not s_5_35
        let s_5_36: bool = !s_5_35;
        // N s_5_37: branch s_5_36 b45 b6
        if s_5_36 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_6_0: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_7_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_8_0: read-var bottom:i64
        let s_8_0: i64 = fn_state.bottom;
        // D s_8_1: write-var bottomshadow#553 <= s_8_0
        fn_state.bottomshadow_553 = s_8_0;
        // C s_8_2: const #0s : i
        let s_8_2: i128 = 0;
        // D s_8_3: read-var mask:i
        let s_8_3: i128 = fn_state.mask;
        // D s_8_4: cmp-gt s_8_3 s_8_2
        let s_8_4: bool = ((s_8_3) > (s_8_2));
        // N s_8_5: branch s_8_4 b44 b9
        if s_8_4 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#30151 <= s_9_0
        fn_state.gs_30151 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#30151:u8
        let s_10_0: bool = fn_state.gs_30151;
        // N s_10_1: branch s_10_0 b32 b11
        if s_10_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_12_0: read-var mask:i
        let s_12_0: i128 = fn_state.mask;
        // D s_12_1: write-var maskshadow#554 <= s_12_0
        fn_state.maskshadow_554 = s_12_0;
        // D s_12_2: read-var bottomshadow#553:i64
        let s_12_2: i64 = fn_state.bottomshadow_553;
        // D s_12_3: cast zx s_12_2 -> i
        let s_12_3: i128 = (i128::try_from(s_12_2).unwrap());
        // D s_12_4: read-var maskshadow#554:i
        let s_12_4: i128 = fn_state.maskshadow_554;
        // D s_12_5: cmp-gt s_12_4 s_12_3
        let s_12_5: bool = ((s_12_4) > (s_12_3));
        // N s_12_6: branch s_12_5 b19 b13
        if s_12_5 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_13_0: read-var n:i64
        let s_13_0: i64 = fn_state.n;
        // D s_13_1: call DBGWVR_read(s_13_0)
        let s_13_1: ProductType700c18a878c5601b = DBGWVR_read(state, tracer, s_13_0);
        // D s_13_2: write-var ga#23353 <= s_13_1
        fn_state.ga_23353 = s_13_1;
        // D s_13_3: read-var ga#23353.0:struct
        let s_13_3: u32 = fn_state.ga_23353._0;
        // D s_13_4: read-var vaddress:u32
        let s_13_4: u32 = fn_state.vaddress;
        // D s_13_5: cast zx s_13_4 -> bv
        let s_13_5: Bits = Bits::new(s_13_4 as u128, 32u16);
        // C s_13_6: const #31s : i64
        let s_13_6: i64 = 31;
        // C s_13_7: cast zx s_13_6 -> i
        let s_13_7: i128 = (i128::try_from(s_13_6).unwrap());
        // D s_13_8: read-var bottomshadow#553:i64
        let s_13_8: i64 = fn_state.bottomshadow_553;
        // D s_13_9: cast zx s_13_8 -> i
        let s_13_9: i128 = (i128::try_from(s_13_8).unwrap());
        // D s_13_10: cast zx s_13_3 -> bv
        let s_13_10: Bits = Bits::new(s_13_3 as u128, 32u16);
        // C s_13_11: const #31s : i64
        let s_13_11: i64 = 31;
        // C s_13_12: cast zx s_13_11 -> i
        let s_13_12: i128 = (i128::try_from(s_13_11).unwrap());
        // D s_13_13: read-var bottomshadow#553:i64
        let s_13_13: i64 = fn_state.bottomshadow_553;
        // D s_13_14: cast zx s_13_13 -> i
        let s_13_14: i128 = (i128::try_from(s_13_13).unwrap());
        // D s_13_15: call subrange_subrange_eq(s_13_5, s_13_7, s_13_9, s_13_10, s_13_12, s_13_14)
        let s_13_15: bool = subrange_subrange_eq(
            state,
            tracer,
            s_13_5,
            s_13_7,
            s_13_9,
            s_13_10,
            s_13_12,
            s_13_14,
        );
        // D s_13_16: write-var WVR_match <= s_13_15
        fn_state.WVR_match = s_13_15;
        // N s_13_17: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var WVR_match:u8
        let s_14_0: bool = fn_state.WVR_match;
        // N s_14_1: branch s_14_0 b18 b15
        if s_14_0 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_15_0: const #0u : u8
        let s_15_0: bool = false;
        // D s_15_1: write-var gs#30178 <= s_15_0
        fn_state.gs_30178 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#30178:u8
        let s_16_0: bool = fn_state.gs_30178;
        // D s_16_1: write-var return_value <= s_16_0
        fn_state.return_value = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_17_0: read-var return_value:u8
        let s_17_0: bool = fn_state.return_value;
        // N s_17_1: return s_17_0
        return s_17_0;
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var byte_select_match:u8
        let s_18_0: bool = fn_state.byte_select_match;
        // D s_18_1: write-var gs#30178 <= s_18_0
        fn_state.gs_30178 = s_18_0;
        // N s_18_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_19_0: read-var maskshadow#554:i
        let s_19_0: i128 = fn_state.maskshadow_554;
        // D s_19_1: call __id(s_19_0)
        let s_19_1: i128 = u__id(state, tracer, s_19_0);
        // C s_19_2: const #0s : i
        let s_19_2: i128 = 0;
        // D s_19_3: cmp-le s_19_2 s_19_1
        let s_19_3: bool = ((s_19_2) <= (s_19_1));
        // N s_19_4: branch s_19_3 b28 b20
        if s_19_3 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#30172 <= s_20_0
        fn_state.gs_30172 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var gs#30172:u8
        let s_21_0: bool = fn_state.gs_30172;
        // N s_21_1: assert s_21_0
        let s_21_1: () = assert!(s_21_0);
        // D s_21_2: read-var n:i64
        let s_21_2: i64 = fn_state.n;
        // D s_21_3: call DBGWVR_read(s_21_2)
        let s_21_3: ProductType700c18a878c5601b = DBGWVR_read(state, tracer, s_21_2);
        // D s_21_4: write-var ga#23346 <= s_21_3
        fn_state.ga_23346 = s_21_3;
        // D s_21_5: read-var ga#23346.0:struct
        let s_21_5: u32 = fn_state.ga_23346._0;
        // D s_21_6: read-var vaddress:u32
        let s_21_6: u32 = fn_state.vaddress;
        // D s_21_7: cast zx s_21_6 -> bv
        let s_21_7: Bits = Bits::new(s_21_6 as u128, 32u16);
        // C s_21_8: const #31s : i64
        let s_21_8: i64 = 31;
        // C s_21_9: cast zx s_21_8 -> i
        let s_21_9: i128 = (i128::try_from(s_21_8).unwrap());
        // D s_21_10: cast zx s_21_5 -> bv
        let s_21_10: Bits = Bits::new(s_21_5 as u128, 32u16);
        // C s_21_11: const #31s : i64
        let s_21_11: i64 = 31;
        // C s_21_12: cast zx s_21_11 -> i
        let s_21_12: i128 = (i128::try_from(s_21_11).unwrap());
        // D s_21_13: read-var maskshadow#554:i
        let s_21_13: i128 = fn_state.maskshadow_554;
        // D s_21_14: read-var maskshadow#554:i
        let s_21_14: i128 = fn_state.maskshadow_554;
        // D s_21_15: call subrange_subrange_eq(s_21_7, s_21_9, s_21_13, s_21_10, s_21_12, s_21_14)
        let s_21_15: bool = subrange_subrange_eq(
            state,
            tracer,
            s_21_7,
            s_21_9,
            s_21_13,
            s_21_10,
            s_21_12,
            s_21_14,
        );
        // D s_21_16: write-var WVR_match <= s_21_15
        fn_state.WVR_match = s_21_15;
        // D s_21_17: read-var WVR_match:u8
        let s_21_17: bool = fn_state.WVR_match;
        // N s_21_18: branch s_21_17 b27 b22
        if s_21_17 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#30174 <= s_22_0
        fn_state.gs_30174 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#30174:u8
        let s_23_0: bool = fn_state.gs_30174;
        // N s_23_1: branch s_23_0 b26 b24
        if s_23_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_24_0: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_25_0: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_26_0: const #30u : u32
        let s_26_0: u32 = 30;
        // S s_26_1: call ConstrainUnpredictableBool(s_26_0)
        let s_26_1: bool = ConstrainUnpredictableBool(state, tracer, s_26_0);
        // D s_26_2: write-var WVR_match <= s_26_1
        fn_state.WVR_match = s_26_1;
        // N s_26_3: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var n:i64
        let s_27_0: i64 = fn_state.n;
        // D s_27_1: call DBGWVR_read(s_27_0)
        let s_27_1: ProductType700c18a878c5601b = DBGWVR_read(state, tracer, s_27_0);
        // D s_27_2: write-var ga#23348 <= s_27_1
        fn_state.ga_23348 = s_27_1;
        // D s_27_3: read-var ga#23348.0:struct
        let s_27_3: u32 = fn_state.ga_23348._0;
        // C s_27_4: const #1s : i
        let s_27_4: i128 = 1;
        // D s_27_5: read-var maskshadow#554:i
        let s_27_5: i128 = fn_state.maskshadow_554;
        // D s_27_6: sub s_27_5 s_27_4
        let s_27_6: i128 = ((s_27_5) - (s_27_4));
        // D s_27_7: cast reint s_27_6 -> i64
        let s_27_7: i64 = (s_27_6 as i64);
        // D s_27_8: cast zx s_27_3 -> bv
        let s_27_8: Bits = Bits::new(s_27_3 as u128, 32u16);
        // D s_27_9: cast zx s_27_7 -> i
        let s_27_9: i128 = (i128::try_from(s_27_7).unwrap());
        // D s_27_10: read-var bottomshadow#553:i64
        let s_27_10: i64 = fn_state.bottomshadow_553;
        // D s_27_11: cast zx s_27_10 -> i
        let s_27_11: i128 = (i128::try_from(s_27_10).unwrap());
        // D s_27_12: call is_zero_subrange(s_27_8, s_27_9, s_27_11)
        let s_27_12: bool = is_zero_subrange(state, tracer, s_27_8, s_27_9, s_27_11);
        // D s_27_13: not s_27_12
        let s_27_13: bool = !s_27_12;
        // D s_27_14: write-var gs#30174 <= s_27_13
        fn_state.gs_30174 = s_27_13;
        // N s_27_15: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var maskshadow#554:i
        let s_28_0: i128 = fn_state.maskshadow_554;
        // D s_28_1: call __id(s_28_0)
        let s_28_1: i128 = u__id(state, tracer, s_28_0);
        // C s_28_2: const #31s : i64
        let s_28_2: i64 = 31;
        // C s_28_3: cast zx s_28_2 -> i
        let s_28_3: i128 = (i128::try_from(s_28_2).unwrap());
        // S s_28_4: call __id(s_28_3)
        let s_28_4: i128 = u__id(state, tracer, s_28_3);
        // S s_28_5: cast reint s_28_4 -> i64
        let s_28_5: i64 = (s_28_4 as i64);
        // S s_28_6: cast zx s_28_5 -> i
        let s_28_6: i128 = (i128::try_from(s_28_5).unwrap());
        // D s_28_7: cmp-le s_28_1 s_28_6
        let s_28_7: bool = ((s_28_1) <= (s_28_6));
        // N s_28_8: branch s_28_7 b31 b29
        if s_28_7 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_29_0: const #0u : u8
        let s_29_0: bool = false;
        // D s_29_1: write-var gs#30171 <= s_29_0
        fn_state.gs_30171 = s_29_0;
        // N s_29_2: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_30_0: read-var gs#30171:u8
        let s_30_0: bool = fn_state.gs_30171;
        // D s_30_1: write-var gs#30172 <= s_30_0
        fn_state.gs_30172 = s_30_0;
        // N s_30_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_31_0: const #31s : i64
        let s_31_0: i64 = 31;
        // C s_31_1: cast zx s_31_0 -> i
        let s_31_1: i128 = (i128::try_from(s_31_0).unwrap());
        // S s_31_2: call __id(s_31_1)
        let s_31_2: i128 = u__id(state, tracer, s_31_1);
        // S s_31_3: cast reint s_31_2 -> i64
        let s_31_3: i64 = (s_31_2 as i64);
        // C s_31_4: const #32s : i
        let s_31_4: i128 = 32;
        // S s_31_5: cast zx s_31_3 -> i
        let s_31_5: i128 = (i128::try_from(s_31_3).unwrap());
        // S s_31_6: cmp-lt s_31_5 s_31_4
        let s_31_6: bool = ((s_31_5) < (s_31_4));
        // D s_31_7: write-var gs#30171 <= s_31_6
        fn_state.gs_30171 = s_31_6;
        // N s_31_8: jump b30
        return block_30(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_32_0: const #3s : i
        let s_32_0: i128 = 3;
        // C s_32_1: const #31s : i
        let s_32_1: i128 = 31;
        // C s_32_2: const #29u : u32
        let s_32_2: u32 = 29;
        // S s_32_3: call ConstrainUnpredictableInteger(s_32_0, s_32_1, s_32_2)
        let s_32_3: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_32_0,
            s_32_1,
            s_32_2,
        );
        // D s_32_4: write-var ga#23334 <= s_32_3
        fn_state.ga_23334 = s_32_3;
        // D s_32_5: read-var ga#23334.0:struct
        let s_32_5: u32 = fn_state.ga_23334._0;
        // D s_32_6: read-var ga#23334.1:struct
        let s_32_6: i128 = fn_state.ga_23334._1;
        // D s_32_7: write-var c <= s_32_5
        fn_state.c = s_32_5;
        // D s_32_8: write-var mask <= s_32_6
        fn_state.mask = s_32_6;
        // D s_32_9: read-var c:u32
        let s_32_9: u32 = fn_state.c;
        // C s_32_10: const #7u : u32
        let s_32_10: u32 = 7;
        // D s_32_11: cmp-eq s_32_9 s_32_10
        let s_32_11: bool = ((s_32_9) == (s_32_10));
        // N s_32_12: branch s_32_11 b43 b33
        if s_32_11 {
            return block_43(state, tracer, fn_state);
        } else {
            return block_33(state, tracer, fn_state);
        };
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var c:u32
        let s_33_0: u32 = fn_state.c;
        // C s_33_1: const #0u : u32
        let s_33_1: u32 = 0;
        // D s_33_2: cmp-eq s_33_0 s_33_1
        let s_33_2: bool = ((s_33_0) == (s_33_1));
        // N s_33_3: branch s_33_2 b42 b34
        if s_33_2 {
            return block_42(state, tracer, fn_state);
        } else {
            return block_34(state, tracer, fn_state);
        };
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_34_0: read-var c:u32
        let s_34_0: u32 = fn_state.c;
        // C s_34_1: const #1u : u32
        let s_34_1: u32 = 1;
        // D s_34_2: cmp-eq s_34_0 s_34_1
        let s_34_2: bool = ((s_34_0) == (s_34_1));
        // D s_34_3: write-var gs#30158 <= s_34_2
        fn_state.gs_30158 = s_34_2;
        // N s_34_4: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var gs#30158:u8
        let s_35_0: bool = fn_state.gs_30158;
        // D s_35_1: write-var gs#30159 <= s_35_0
        fn_state.gs_30159 = s_35_0;
        // N s_35_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_36_0: read-var gs#30159:u8
        let s_36_0: bool = fn_state.gs_30159;
        // N s_36_1: assert s_36_0
        let s_36_1: () = assert!(s_36_0);
        // C s_36_2: const #7u : u32
        let s_36_2: u32 = 7;
        // D s_36_3: read-var c:u32
        let s_36_3: u32 = fn_state.c;
        // D s_36_4: cmp-eq s_36_2 s_36_3
        let s_36_4: bool = ((s_36_2) == (s_36_3));
        // D s_36_5: not s_36_4
        let s_36_5: bool = !s_36_4;
        // N s_36_6: branch s_36_5 b38 b37
        if s_36_5 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_37(state, tracer, fn_state);
        };
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_37_0: const #0u : u8
        let s_37_0: bool = false;
        // D s_37_1: write-var return_value <= s_37_0
        fn_state.return_value = s_37_0;
        // N s_37_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_38_0: const #0u : u32
        let s_38_0: u32 = 0;
        // D s_38_1: read-var c:u32
        let s_38_1: u32 = fn_state.c;
        // D s_38_2: cmp-eq s_38_0 s_38_1
        let s_38_2: bool = ((s_38_0) == (s_38_1));
        // D s_38_3: not s_38_2
        let s_38_3: bool = !s_38_2;
        // N s_38_4: branch s_38_3 b41 b39
        if s_38_3 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_39(state, tracer, fn_state);
        };
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_39_0: const #0s : i
        let s_39_0: i128 = 0;
        // D s_39_1: write-var mask <= s_39_0
        fn_state.mask = s_39_0;
        // N s_39_2: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_40_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_41_0: jump b40
        return block_40(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_42_0: const #1u : u8
        let s_42_0: bool = true;
        // D s_42_1: write-var gs#30158 <= s_42_0
        fn_state.gs_30158 = s_42_0;
        // N s_42_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #1u : u8
        let s_43_0: bool = true;
        // D s_43_1: write-var gs#30159 <= s_43_0
        fn_state.gs_30159 = s_43_0;
        // N s_43_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_44_0: const #2s : i
        let s_44_0: i128 = 2;
        // D s_44_1: read-var mask:i
        let s_44_1: i128 = fn_state.mask;
        // D s_44_2: cmp-le s_44_1 s_44_0
        let s_44_2: bool = ((s_44_1) <= (s_44_0));
        // D s_44_3: write-var gs#30151 <= s_44_2
        fn_state.gs_30151 = s_44_2;
        // N s_44_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_45_0: const #28u : u32
        let s_45_0: u32 = 28;
        // S s_45_1: call ConstrainUnpredictableBool(s_45_0)
        let s_45_1: bool = ConstrainUnpredictableBool(state, tracer, s_45_0);
        // D s_45_2: write-var byte_select_match <= s_45_1
        fn_state.byte_select_match = s_45_1;
        // C s_45_3: const #3s : i64
        let s_45_3: i64 = 3;
        // D s_45_4: write-var bottom <= s_45_3
        fn_state.bottom = s_45_3;
        // N s_45_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #27u : u32
        let s_46_0: u32 = 27;
        // S s_46_1: call ConstrainUnpredictableBool(s_46_0)
        let s_46_1: bool = ConstrainUnpredictableBool(state, tracer, s_46_0);
        // D s_46_2: write-var byte_select_match <= s_46_1
        fn_state.byte_select_match = s_46_1;
        // N s_46_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_47_0: read-var n:i64
        let s_47_0: i64 = fn_state.n;
        // D s_47_1: call DBGWCR_read(s_47_0)
        let s_47_1: ProductType700c18a878c5601b = DBGWCR_read(state, tracer, s_47_0);
        // D s_47_2: call _get_DBGWCR_Type_BAS(s_47_1)
        let s_47_2: u8 = u_get_DBGWCR_Type_BAS(state, tracer, s_47_1);
        // D s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 8u16);
        // D s_47_4: call IsOnes(s_47_3)
        let s_47_4: bool = IsOnes(state, tracer, s_47_3);
        // D s_47_5: not s_47_4
        let s_47_5: bool = !s_47_4;
        // D s_47_6: write-var gs#30141 <= s_47_5
        fn_state.gs_30141 = s_47_5;
        // N s_47_7: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_48_0: const #2s : i64
        let s_48_0: i64 = 2;
        // D s_48_1: write-var bottom <= s_48_0
        fn_state.bottom = s_48_0;
        // N s_48_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
