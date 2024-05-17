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
use unsigned_subrange::*;
use u_get_DBGWCR_EL1_Type_MASK::*;
use IsZero::*;
use u__id::*;
use u_get_DBGWCR_EL1_Type_BAS::*;
use ConstrainUnpredictableBool::*;
use is_ones_subrange::*;
use ConstrainUnpredictableInteger::*;
use subrange_subrange_eq::*;
use IsOnes::*;
use DebugAddrTop::*;
use is_zero_subrange::*;
use common::*;
pub fn AArch64_WatchpointByteMatch<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    vaddress: u64,
) -> bool {
    #[derive(Default)]
    struct FunctionState {
        gs_16651: bool,
        gs_16643: bool,
        gs_16662: bool,
        topshadow_282: i128,
        gs_16656: bool,
        gs_16628: bool,
        gs_16642: bool,
        ga_12413: ProductType5c790c8ef59cc8b2,
        c: u32,
        mask: i128,
        bottomshadow_280: i64,
        maskshadow_281: i128,
        byte_select_match: bool,
        gs_16650: bool,
        ga_12389: ProductType396b95aa74979079,
        gs_16655: bool,
        gs_16621: bool,
        return_value: bool,
        bottom: i64,
        WVR_match: bool,
        ga_12411: ProductType5c790c8ef59cc8b2,
        gs_16629: bool,
        top: i128,
        gs_16640: bool,
        ga_12436: ProductType5c790c8ef59cc8b2,
        ga_12425: ProductType5c790c8ef59cc8b2,
        gs_16611: bool,
        gs_16658: bool,
        ga_12397: ProductType5c790c8ef59cc8b2,
        ga_12393: ProductType5c790c8ef59cc8b2,
        n: i64,
        vaddress: u64,
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
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call DebugAddrTop(s_0_0)
        let s_0_1: i128 = DebugAddrTop(state, tracer, s_0_0);
        // D s_0_2: write-var top <= s_0_1
        fn_state.top = s_0_1;
        // C s_0_3: const #19392u : u32
        let s_0_3: u32 = 19392;
        // D s_0_4: read-reg s_0_3:[struct; 64]
        let s_0_4: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_0_3 as isize);
            tracer.read_register(s_0_3 as isize, value);
            value
        };
        // D s_0_5: read-var n:i64
        let s_0_5: i64 = fn_state.n;
        // D s_0_6: cast zx s_0_5 -> i
        let s_0_6: i128 = (i128::try_from(s_0_5).unwrap());
        // D s_0_7: read-element s_0_4[s_0_6]
        let s_0_7: ProductType5c790c8ef59cc8b2 = s_0_4[(s_0_6) as usize];
        // D s_0_8: write-var ga#12436 <= s_0_7
        fn_state.ga_12436 = s_0_7;
        // D s_0_9: read-var ga#12436.0:struct
        let s_0_9: u64 = fn_state.ga_12436._0;
        // C s_0_10: const #2s : i
        let s_0_10: i128 = 2;
        // D s_0_11: cast zx s_0_9 -> bv
        let s_0_11: Bits = Bits::new(s_0_9 as u128, 64u16);
        // C s_0_12: const #1u : u64
        let s_0_12: u64 = 1;
        // D s_0_13: bit-extract s_0_11 s_0_10 s_0_12
        let s_0_13: Bits = (Bits::new(
            ((s_0_11) >> (s_0_10)).value(),
            u16::try_from(s_0_12).unwrap(),
        ));
        // D s_0_14: cast reint s_0_13 -> u8
        let s_0_14: bool = ((s_0_13.value()) != 0);
        // C s_0_15: const #0s : i
        let s_0_15: i128 = 0;
        // C s_0_16: const #0u : u64
        let s_0_16: u64 = 0;
        // D s_0_17: cast zx s_0_14 -> u64
        let s_0_17: u64 = (s_0_14 as u64);
        // C s_0_18: const #1u : u64
        let s_0_18: u64 = 1;
        // D s_0_19: and s_0_17 s_0_18
        let s_0_19: u64 = ((s_0_17) & (s_0_18));
        // D s_0_20: cmp-eq s_0_19 s_0_18
        let s_0_20: bool = ((s_0_19) == (s_0_18));
        // D s_0_21: lsl s_0_17 s_0_15
        let s_0_21: u64 = s_0_17 << s_0_15;
        // D s_0_22: or s_0_16 s_0_21
        let s_0_22: u64 = ((s_0_16) | (s_0_21));
        // D s_0_23: cmpl s_0_21
        let s_0_23: u64 = !s_0_21;
        // D s_0_24: and s_0_16 s_0_23
        let s_0_24: u64 = ((s_0_16) & (s_0_23));
        // D s_0_25: select s_0_20 s_0_22 s_0_24
        let s_0_25: u64 = if s_0_20 { s_0_22 } else { s_0_24 };
        // D s_0_26: cast trunc s_0_25 -> u8
        let s_0_26: bool = ((s_0_25) != 0);
        // D s_0_27: cast zx s_0_26 -> bv
        let s_0_27: Bits = Bits::new(s_0_26 as u128, 1u16);
        // C s_0_28: const #1u : u8
        let s_0_28: bool = true;
        // C s_0_29: cast zx s_0_28 -> bv
        let s_0_29: Bits = Bits::new(s_0_28 as u128, 1u16);
        // D s_0_30: cmp-eq s_0_27 s_0_29
        let s_0_30: bool = ((s_0_27) == (s_0_29));
        // N s_0_31: branch s_0_30 b66 b1
        if s_0_30 {
            return block_66(state, tracer, fn_state);
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
        // C s_2_1: const #103984u : u32
        let s_2_1: u32 = 103984;
        // D s_2_2: read-reg s_2_1:[struct; 64]
        let s_2_2: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_2_1 as isize);
            tracer.read_register(s_2_1 as isize, value);
            value
        };
        // D s_2_3: read-var n:i64
        let s_2_3: i64 = fn_state.n;
        // D s_2_4: cast zx s_2_3 -> i
        let s_2_4: i128 = (i128::try_from(s_2_3).unwrap());
        // D s_2_5: read-element s_2_2[s_2_4]
        let s_2_5: ProductType5c790c8ef59cc8b2 = s_2_2[(s_2_4) as usize];
        // D s_2_6: call _get_DBGWCR_EL1_Type_BAS(s_2_5)
        let s_2_6: u8 = u_get_DBGWCR_EL1_Type_BAS(state, tracer, s_2_5);
        // C s_2_7: const #1s : i
        let s_2_7: i128 = 1;
        // D s_2_8: cast zx s_2_0 -> i
        let s_2_8: i128 = (i128::try_from(s_2_0).unwrap());
        // D s_2_9: sub s_2_8 s_2_7
        let s_2_9: i128 = ((s_2_8) - (s_2_7));
        // D s_2_10: cast reint s_2_9 -> i64
        let s_2_10: i64 = (s_2_9 as i64);
        // C s_2_11: const #0s : i
        let s_2_11: i128 = 0;
        // D s_2_12: read-var vaddress:u64
        let s_2_12: u64 = fn_state.vaddress;
        // D s_2_13: cast zx s_2_12 -> bv
        let s_2_13: Bits = Bits::new(s_2_12 as u128, 64u16);
        // D s_2_14: cast zx s_2_10 -> i
        let s_2_14: i128 = (i128::try_from(s_2_10).unwrap());
        // D s_2_15: call unsigned_subrange(s_2_13, s_2_14, s_2_11)
        let s_2_15: i128 = unsigned_subrange(state, tracer, s_2_13, s_2_14, s_2_11);
        // D s_2_16: cast reint s_2_15 -> i64
        let s_2_16: i64 = (s_2_15 as i64);
        // D s_2_17: cast zx s_2_6 -> bv
        let s_2_17: Bits = Bits::new(s_2_6 as u128, 8u16);
        // D s_2_18: cast zx s_2_16 -> i
        let s_2_18: i128 = (i128::try_from(s_2_16).unwrap());
        // C s_2_19: const #1u : u64
        let s_2_19: u64 = 1;
        // D s_2_20: bit-extract s_2_17 s_2_18 s_2_19
        let s_2_20: Bits = (Bits::new(
            ((s_2_17) >> (s_2_18)).value(),
            u16::try_from(s_2_19).unwrap(),
        ));
        // D s_2_21: cast reint s_2_20 -> u8
        let s_2_21: bool = ((s_2_20.value()) != 0);
        // C s_2_22: const #0s : i
        let s_2_22: i128 = 0;
        // C s_2_23: const #0u : u64
        let s_2_23: u64 = 0;
        // D s_2_24: cast zx s_2_21 -> u64
        let s_2_24: u64 = (s_2_21 as u64);
        // C s_2_25: const #1u : u64
        let s_2_25: u64 = 1;
        // D s_2_26: and s_2_24 s_2_25
        let s_2_26: u64 = ((s_2_24) & (s_2_25));
        // D s_2_27: cmp-eq s_2_26 s_2_25
        let s_2_27: bool = ((s_2_26) == (s_2_25));
        // D s_2_28: lsl s_2_24 s_2_22
        let s_2_28: u64 = s_2_24 << s_2_22;
        // D s_2_29: or s_2_23 s_2_28
        let s_2_29: u64 = ((s_2_23) | (s_2_28));
        // D s_2_30: cmpl s_2_28
        let s_2_30: u64 = !s_2_28;
        // D s_2_31: and s_2_23 s_2_30
        let s_2_31: u64 = ((s_2_23) & (s_2_30));
        // D s_2_32: select s_2_27 s_2_29 s_2_31
        let s_2_32: u64 = if s_2_27 { s_2_29 } else { s_2_31 };
        // D s_2_33: cast trunc s_2_32 -> u8
        let s_2_33: bool = ((s_2_32) != 0);
        // D s_2_34: cast zx s_2_33 -> bv
        let s_2_34: Bits = Bits::new(s_2_33 as u128, 1u16);
        // C s_2_35: const #0u : u8
        let s_2_35: bool = false;
        // C s_2_36: cast zx s_2_35 -> bv
        let s_2_36: Bits = Bits::new(s_2_35 as u128, 1u16);
        // D s_2_37: cmp-ne s_2_34 s_2_36
        let s_2_37: bool = ((s_2_34) != (s_2_36));
        // D s_2_38: write-var byte_select_match <= s_2_37
        fn_state.byte_select_match = s_2_37;
        // C s_2_39: const #103984u : u32
        let s_2_39: u32 = 103984;
        // D s_2_40: read-reg s_2_39:[struct; 64]
        let s_2_40: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_2_39 as isize);
            tracer.read_register(s_2_39 as isize, value);
            value
        };
        // D s_2_41: read-var n:i64
        let s_2_41: i64 = fn_state.n;
        // D s_2_42: cast zx s_2_41 -> i
        let s_2_42: i128 = (i128::try_from(s_2_41).unwrap());
        // D s_2_43: read-element s_2_40[s_2_42]
        let s_2_43: ProductType5c790c8ef59cc8b2 = s_2_40[(s_2_42) as usize];
        // D s_2_44: call _get_DBGWCR_EL1_Type_MASK(s_2_43)
        let s_2_44: u8 = u_get_DBGWCR_EL1_Type_MASK(state, tracer, s_2_43);
        // D s_2_45: cast zx s_2_44 -> bv
        let s_2_45: Bits = Bits::new(s_2_44 as u128, 5u16);
        // D s_2_46: cast zx s_2_45 -> i
        let s_2_46: i128 = (s_2_45.value() as i128);
        // D s_2_47: write-var mask <= s_2_46
        fn_state.mask = s_2_46;
        // C s_2_48: const #0s : i
        let s_2_48: i128 = 0;
        // D s_2_49: read-var mask:i
        let s_2_49: i128 = fn_state.mask;
        // D s_2_50: cmp-gt s_2_49 s_2_48
        let s_2_50: bool = ((s_2_49) > (s_2_48));
        // N s_2_51: branch s_2_50 b65 b3
        if s_2_50 {
            return block_65(state, tracer, fn_state);
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
        // D s_3_1: write-var gs#16611 <= s_3_0
        fn_state.gs_16611 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_4_0: read-var gs#16611:u8
        let s_4_0: bool = fn_state.gs_16611;
        // N s_4_1: branch s_4_0 b64 b5
        if s_4_0 {
            return block_64(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_5_0: const #103984u : u32
        let s_5_0: u32 = 103984;
        // D s_5_1: read-reg s_5_0:[struct; 64]
        let s_5_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: read-var n:i64
        let s_5_2: i64 = fn_state.n;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-element s_5_1[s_5_3]
        let s_5_4: ProductType5c790c8ef59cc8b2 = s_5_1[(s_5_3) as usize];
        // D s_5_5: call _get_DBGWCR_EL1_Type_BAS(s_5_4)
        let s_5_5: u8 = u_get_DBGWCR_EL1_Type_BAS(state, tracer, s_5_4);
        // C s_5_6: const #103984u : u32
        let s_5_6: u32 = 103984;
        // D s_5_7: read-reg s_5_6:[struct; 64]
        let s_5_7: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 64usize]>(s_5_6 as isize);
            tracer.read_register(s_5_6 as isize, value);
            value
        };
        // D s_5_8: read-var n:i64
        let s_5_8: i64 = fn_state.n;
        // D s_5_9: cast zx s_5_8 -> i
        let s_5_9: i128 = (i128::try_from(s_5_8).unwrap());
        // D s_5_10: read-element s_5_7[s_5_9]
        let s_5_10: ProductType5c790c8ef59cc8b2 = s_5_7[(s_5_9) as usize];
        // D s_5_11: call _get_DBGWCR_EL1_Type_BAS(s_5_10)
        let s_5_11: u8 = u_get_DBGWCR_EL1_Type_BAS(state, tracer, s_5_10);
        // C s_5_12: const #1s : i
        let s_5_12: i128 = 1;
        // D s_5_13: cast zx s_5_11 -> bv
        let s_5_13: Bits = Bits::new(s_5_11 as u128, 8u16);
        // C s_5_14: cast cvt s_5_12 -> bv
        let s_5_14: Bits = Bits::new(s_5_12 as u128, 128);
        // D s_5_15: sub s_5_13 s_5_14
        let s_5_15: Bits = ((s_5_13) - (s_5_14));
        // D s_5_16: cast reint s_5_15 -> u8
        let s_5_16: u8 = (s_5_15.value() as u8);
        // D s_5_17: cast zx s_5_16 -> bv
        let s_5_17: Bits = Bits::new(s_5_16 as u128, 8u16);
        // D s_5_18: not s_5_17
        let s_5_18: Bits = !s_5_17;
        // D s_5_19: cast reint s_5_18 -> u8
        let s_5_19: u8 = (s_5_18.value() as u8);
        // D s_5_20: cast zx s_5_5 -> bv
        let s_5_20: Bits = Bits::new(s_5_5 as u128, 8u16);
        // D s_5_21: cast zx s_5_19 -> bv
        let s_5_21: Bits = Bits::new(s_5_19 as u128, 8u16);
        // D s_5_22: and s_5_20 s_5_21
        let s_5_22: Bits = ((s_5_20) & (s_5_21));
        // D s_5_23: cast reint s_5_22 -> u8
        let s_5_23: u8 = (s_5_22.value() as u8);
        // C s_5_24: const #103984u : u32
        let s_5_24: u32 = 103984;
        // D s_5_25: read-reg s_5_24:[struct; 64]
        let s_5_25: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_5_24 as isize);
            tracer.read_register(s_5_24 as isize, value);
            value
        };
        // D s_5_26: read-var n:i64
        let s_5_26: i64 = fn_state.n;
        // D s_5_27: cast zx s_5_26 -> i
        let s_5_27: i128 = (i128::try_from(s_5_26).unwrap());
        // D s_5_28: read-element s_5_25[s_5_27]
        let s_5_28: ProductType5c790c8ef59cc8b2 = s_5_25[(s_5_27) as usize];
        // D s_5_29: call _get_DBGWCR_EL1_Type_BAS(s_5_28)
        let s_5_29: u8 = u_get_DBGWCR_EL1_Type_BAS(state, tracer, s_5_28);
        // D s_5_30: cast zx s_5_29 -> bv
        let s_5_30: Bits = Bits::new(s_5_29 as u128, 8u16);
        // D s_5_31: cast zx s_5_23 -> bv
        let s_5_31: Bits = Bits::new(s_5_23 as u128, 8u16);
        // D s_5_32: add s_5_30 s_5_31
        let s_5_32: Bits = (s_5_30 + s_5_31);
        // D s_5_33: cast reint s_5_32 -> u8
        let s_5_33: u8 = (s_5_32.value() as u8);
        // C s_5_34: const #1s : i
        let s_5_34: i128 = 1;
        // D s_5_35: cast zx s_5_33 -> bv
        let s_5_35: Bits = Bits::new(s_5_33 as u128, 8u16);
        // C s_5_36: cast cvt s_5_34 -> bv
        let s_5_36: Bits = Bits::new(s_5_34 as u128, 128);
        // D s_5_37: sub s_5_35 s_5_36
        let s_5_37: Bits = ((s_5_35) - (s_5_36));
        // D s_5_38: cast reint s_5_37 -> u8
        let s_5_38: u8 = (s_5_37.value() as u8);
        // D s_5_39: cast zx s_5_33 -> bv
        let s_5_39: Bits = Bits::new(s_5_33 as u128, 8u16);
        // D s_5_40: cast zx s_5_38 -> bv
        let s_5_40: Bits = Bits::new(s_5_38 as u128, 8u16);
        // D s_5_41: and s_5_39 s_5_40
        let s_5_41: Bits = ((s_5_39) & (s_5_40));
        // D s_5_42: cast reint s_5_41 -> u8
        let s_5_42: u8 = (s_5_41.value() as u8);
        // D s_5_43: cast zx s_5_42 -> bv
        let s_5_43: Bits = Bits::new(s_5_42 as u128, 8u16);
        // D s_5_44: call IsZero(s_5_43)
        let s_5_44: bool = IsZero(state, tracer, s_5_43);
        // D s_5_45: not s_5_44
        let s_5_45: bool = !s_5_44;
        // N s_5_46: branch s_5_45 b63 b6
        if s_5_45 {
            return block_63(state, tracer, fn_state);
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
        // D s_8_1: write-var bottomshadow#280 <= s_8_0
        fn_state.bottomshadow_280 = s_8_0;
        // C s_8_2: const #0s : i
        let s_8_2: i128 = 0;
        // D s_8_3: read-var mask:i
        let s_8_3: i128 = fn_state.mask;
        // D s_8_4: cmp-gt s_8_3 s_8_2
        let s_8_4: bool = ((s_8_3) > (s_8_2));
        // N s_8_5: branch s_8_4 b62 b9
        if s_8_4 {
            return block_62(state, tracer, fn_state);
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
        // D s_9_1: write-var gs#16621 <= s_9_0
        fn_state.gs_16621 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_10_0: read-var gs#16621:u8
        let s_10_0: bool = fn_state.gs_16621;
        // N s_10_1: branch s_10_0 b50 b11
        if s_10_0 {
            return block_50(state, tracer, fn_state);
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
        // D s_12_1: write-var maskshadow#281 <= s_12_0
        fn_state.maskshadow_281 = s_12_0;
        // C s_12_2: const #55s : i
        let s_12_2: i128 = 55;
        // D s_12_3: read-var top:i
        let s_12_3: i128 = fn_state.top;
        // D s_12_4: cmp-lt s_12_3 s_12_2
        let s_12_4: bool = ((s_12_3) < (s_12_2));
        // N s_12_5: branch s_12_4 b49 b13
        if s_12_4 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_13(state, tracer, fn_state);
        };
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_13_0: const #0u : u8
        let s_13_0: bool = false;
        // D s_13_1: write-var gs#16640 <= s_13_0
        fn_state.gs_16640 = s_13_0;
        // N s_13_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_14_0: read-var gs#16640:u8
        let s_14_0: bool = fn_state.gs_16640;
        // N s_14_1: branch s_14_0 b48 b15
        if s_14_0 {
            return block_48(state, tracer, fn_state);
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
        // D s_15_1: write-var gs#16642 <= s_15_0
        fn_state.gs_16642 = s_15_0;
        // N s_15_2: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_16_0: read-var gs#16642:u8
        let s_16_0: bool = fn_state.gs_16642;
        // N s_16_1: branch s_16_0 b47 b17
        if s_16_0 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var gs#16643 <= s_17_0
        fn_state.gs_16643 = s_17_0;
        // N s_17_2: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_18_0: read-var gs#16643:u8
        let s_18_0: bool = fn_state.gs_16643;
        // N s_18_1: branch s_18_0 b46 b19
        if s_18_0 {
            return block_46(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_19_0: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_20_0: read-var top:i
        let s_20_0: i128 = fn_state.top;
        // D s_20_1: write-var topshadow#282 <= s_20_0
        fn_state.topshadow_282 = s_20_0;
        // D s_20_2: read-var bottomshadow#280:i64
        let s_20_2: i64 = fn_state.bottomshadow_280;
        // D s_20_3: cast zx s_20_2 -> i
        let s_20_3: i128 = (i128::try_from(s_20_2).unwrap());
        // D s_20_4: read-var maskshadow#281:i
        let s_20_4: i128 = fn_state.maskshadow_281;
        // D s_20_5: cmp-gt s_20_4 s_20_3
        let s_20_5: bool = ((s_20_4) > (s_20_3));
        // N s_20_6: branch s_20_5 b33 b21
        if s_20_5 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_21_0: read-var bottomshadow#280:i64
        let s_21_0: i64 = fn_state.bottomshadow_280;
        // D s_21_1: cast zx s_21_0 -> i
        let s_21_1: i128 = (i128::try_from(s_21_0).unwrap());
        // D s_21_2: call __id(s_21_1)
        let s_21_2: i128 = u__id(state, tracer, s_21_1);
        // D s_21_3: cast reint s_21_2 -> i64
        let s_21_3: i64 = (s_21_2 as i64);
        // C s_21_4: const #0s : i
        let s_21_4: i128 = 0;
        // D s_21_5: cast zx s_21_3 -> i
        let s_21_5: i128 = (i128::try_from(s_21_3).unwrap());
        // D s_21_6: cmp-le s_21_4 s_21_5
        let s_21_6: bool = ((s_21_4) <= (s_21_5));
        // N s_21_7: branch s_21_6 b29 b22
        if s_21_6 {
            return block_29(state, tracer, fn_state);
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
        // D s_22_1: write-var gs#16651 <= s_22_0
        fn_state.gs_16651 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_23_0: read-var gs#16651:u8
        let s_23_0: bool = fn_state.gs_16651;
        // N s_23_1: assert s_23_0
        let s_23_1: () = assert!(s_23_0);
        // C s_23_2: const #19392u : u32
        let s_23_2: u32 = 19392;
        // D s_23_3: read-reg s_23_2:[struct; 64]
        let s_23_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_23_2 as isize);
            tracer.read_register(s_23_2 as isize, value);
            value
        };
        // D s_23_4: read-var n:i64
        let s_23_4: i64 = fn_state.n;
        // D s_23_5: cast zx s_23_4 -> i
        let s_23_5: i128 = (i128::try_from(s_23_4).unwrap());
        // D s_23_6: read-element s_23_3[s_23_5]
        let s_23_6: ProductType5c790c8ef59cc8b2 = s_23_3[(s_23_5) as usize];
        // D s_23_7: write-var ga#12425 <= s_23_6
        fn_state.ga_12425 = s_23_6;
        // D s_23_8: read-var ga#12425.0:struct
        let s_23_8: u64 = fn_state.ga_12425._0;
        // D s_23_9: read-var vaddress:u64
        let s_23_9: u64 = fn_state.vaddress;
        // D s_23_10: cast zx s_23_9 -> bv
        let s_23_10: Bits = Bits::new(s_23_9 as u128, 64u16);
        // D s_23_11: read-var bottomshadow#280:i64
        let s_23_11: i64 = fn_state.bottomshadow_280;
        // D s_23_12: cast zx s_23_11 -> i
        let s_23_12: i128 = (i128::try_from(s_23_11).unwrap());
        // D s_23_13: cast zx s_23_8 -> bv
        let s_23_13: Bits = Bits::new(s_23_8 as u128, 64u16);
        // D s_23_14: read-var bottomshadow#280:i64
        let s_23_14: i64 = fn_state.bottomshadow_280;
        // D s_23_15: cast zx s_23_14 -> i
        let s_23_15: i128 = (i128::try_from(s_23_14).unwrap());
        // D s_23_16: read-var topshadow#282:i
        let s_23_16: i128 = fn_state.topshadow_282;
        // D s_23_17: read-var topshadow#282:i
        let s_23_17: i128 = fn_state.topshadow_282;
        // D s_23_18: call subrange_subrange_eq(s_23_10, s_23_16, s_23_12, s_23_13, s_23_17, s_23_15)
        let s_23_18: bool = subrange_subrange_eq(
            state,
            tracer,
            s_23_10,
            s_23_16,
            s_23_12,
            s_23_13,
            s_23_17,
            s_23_15,
        );
        // D s_23_19: write-var WVR_match <= s_23_18
        fn_state.WVR_match = s_23_18;
        // N s_23_20: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_24_0: read-var WVR_match:u8
        let s_24_0: bool = fn_state.WVR_match;
        // N s_24_1: branch s_24_0 b28 b25
        if s_24_0 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_25(state, tracer, fn_state);
        };
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_25_0: const #0u : u8
        let s_25_0: bool = false;
        // D s_25_1: write-var gs#16662 <= s_25_0
        fn_state.gs_16662 = s_25_0;
        // N s_25_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_26_0: read-var gs#16662:u8
        let s_26_0: bool = fn_state.gs_16662;
        // D s_26_1: write-var return_value <= s_26_0
        fn_state.return_value = s_26_0;
        // N s_26_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_27_0: read-var return_value:u8
        let s_27_0: bool = fn_state.return_value;
        // N s_27_1: return s_27_0
        return s_27_0;
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_28_0: read-var byte_select_match:u8
        let s_28_0: bool = fn_state.byte_select_match;
        // D s_28_1: write-var gs#16662 <= s_28_0
        fn_state.gs_16662 = s_28_0;
        // N s_28_2: jump b26
        return block_26(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_29_0: read-var bottomshadow#280:i64
        let s_29_0: i64 = fn_state.bottomshadow_280;
        // D s_29_1: cast zx s_29_0 -> i
        let s_29_1: i128 = (i128::try_from(s_29_0).unwrap());
        // D s_29_2: call __id(s_29_1)
        let s_29_2: i128 = u__id(state, tracer, s_29_1);
        // D s_29_3: cast reint s_29_2 -> i64
        let s_29_3: i64 = (s_29_2 as i64);
        // D s_29_4: read-var topshadow#282:i
        let s_29_4: i128 = fn_state.topshadow_282;
        // D s_29_5: call __id(s_29_4)
        let s_29_5: i128 = u__id(state, tracer, s_29_4);
        // D s_29_6: cast zx s_29_3 -> i
        let s_29_6: i128 = (i128::try_from(s_29_3).unwrap());
        // D s_29_7: cmp-le s_29_6 s_29_5
        let s_29_7: bool = ((s_29_6) <= (s_29_5));
        // N s_29_8: branch s_29_7 b32 b30
        if s_29_7 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_30_0: const #0u : u8
        let s_30_0: bool = false;
        // D s_30_1: write-var gs#16650 <= s_30_0
        fn_state.gs_16650 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_31_0: read-var gs#16650:u8
        let s_31_0: bool = fn_state.gs_16650;
        // D s_31_1: write-var gs#16651 <= s_31_0
        fn_state.gs_16651 = s_31_0;
        // N s_31_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_32_0: read-var topshadow#282:i
        let s_32_0: i128 = fn_state.topshadow_282;
        // D s_32_1: call __id(s_32_0)
        let s_32_1: i128 = u__id(state, tracer, s_32_0);
        // C s_32_2: const #64s : i
        let s_32_2: i128 = 64;
        // D s_32_3: cmp-lt s_32_1 s_32_2
        let s_32_3: bool = ((s_32_1) < (s_32_2));
        // D s_32_4: write-var gs#16650 <= s_32_3
        fn_state.gs_16650 = s_32_3;
        // N s_32_5: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_33_0: read-var maskshadow#281:i
        let s_33_0: i128 = fn_state.maskshadow_281;
        // D s_33_1: call __id(s_33_0)
        let s_33_1: i128 = u__id(state, tracer, s_33_0);
        // C s_33_2: const #0s : i
        let s_33_2: i128 = 0;
        // D s_33_3: cmp-le s_33_2 s_33_1
        let s_33_3: bool = ((s_33_2) <= (s_33_1));
        // N s_33_4: branch s_33_3 b42 b34
        if s_33_3 {
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
        // C s_34_0: const #0u : u8
        let s_34_0: bool = false;
        // D s_34_1: write-var gs#16656 <= s_34_0
        fn_state.gs_16656 = s_34_0;
        // N s_34_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_35_0: read-var gs#16656:u8
        let s_35_0: bool = fn_state.gs_16656;
        // N s_35_1: assert s_35_0
        let s_35_1: () = assert!(s_35_0);
        // C s_35_2: const #19392u : u32
        let s_35_2: u32 = 19392;
        // D s_35_3: read-reg s_35_2:[struct; 64]
        let s_35_3: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_35_2 as isize);
            tracer.read_register(s_35_2 as isize, value);
            value
        };
        // D s_35_4: read-var n:i64
        let s_35_4: i64 = fn_state.n;
        // D s_35_5: cast zx s_35_4 -> i
        let s_35_5: i128 = (i128::try_from(s_35_4).unwrap());
        // D s_35_6: read-element s_35_3[s_35_5]
        let s_35_6: ProductType5c790c8ef59cc8b2 = s_35_3[(s_35_5) as usize];
        // D s_35_7: write-var ga#12411 <= s_35_6
        fn_state.ga_12411 = s_35_6;
        // D s_35_8: read-var ga#12411.0:struct
        let s_35_8: u64 = fn_state.ga_12411._0;
        // D s_35_9: read-var vaddress:u64
        let s_35_9: u64 = fn_state.vaddress;
        // D s_35_10: cast zx s_35_9 -> bv
        let s_35_10: Bits = Bits::new(s_35_9 as u128, 64u16);
        // D s_35_11: cast zx s_35_8 -> bv
        let s_35_11: Bits = Bits::new(s_35_8 as u128, 64u16);
        // D s_35_12: read-var topshadow#282:i
        let s_35_12: i128 = fn_state.topshadow_282;
        // D s_35_13: read-var maskshadow#281:i
        let s_35_13: i128 = fn_state.maskshadow_281;
        // D s_35_14: read-var topshadow#282:i
        let s_35_14: i128 = fn_state.topshadow_282;
        // D s_35_15: read-var maskshadow#281:i
        let s_35_15: i128 = fn_state.maskshadow_281;
        // D s_35_16: call subrange_subrange_eq(s_35_10, s_35_12, s_35_13, s_35_11, s_35_14, s_35_15)
        let s_35_16: bool = subrange_subrange_eq(
            state,
            tracer,
            s_35_10,
            s_35_12,
            s_35_13,
            s_35_11,
            s_35_14,
            s_35_15,
        );
        // D s_35_17: write-var WVR_match <= s_35_16
        fn_state.WVR_match = s_35_16;
        // D s_35_18: read-var WVR_match:u8
        let s_35_18: bool = fn_state.WVR_match;
        // N s_35_19: branch s_35_18 b41 b36
        if s_35_18 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_36(state, tracer, fn_state);
        };
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_36_0: const #0u : u8
        let s_36_0: bool = false;
        // D s_36_1: write-var gs#16658 <= s_36_0
        fn_state.gs_16658 = s_36_0;
        // N s_36_2: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_37_0: read-var gs#16658:u8
        let s_37_0: bool = fn_state.gs_16658;
        // N s_37_1: branch s_37_0 b40 b38
        if s_37_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_38(state, tracer, fn_state);
        };
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_38_0: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_39_0: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_40_0: const #30u : u32
        let s_40_0: u32 = 30;
        // S s_40_1: call ConstrainUnpredictableBool(s_40_0)
        let s_40_1: bool = ConstrainUnpredictableBool(state, tracer, s_40_0);
        // D s_40_2: write-var WVR_match <= s_40_1
        fn_state.WVR_match = s_40_1;
        // N s_40_3: jump b39
        return block_39(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_41_0: const #19392u : u32
        let s_41_0: u32 = 19392;
        // D s_41_1: read-reg s_41_0:[struct; 64]
        let s_41_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_41_0 as isize);
            tracer.read_register(s_41_0 as isize, value);
            value
        };
        // D s_41_2: read-var n:i64
        let s_41_2: i64 = fn_state.n;
        // D s_41_3: cast zx s_41_2 -> i
        let s_41_3: i128 = (i128::try_from(s_41_2).unwrap());
        // D s_41_4: read-element s_41_1[s_41_3]
        let s_41_4: ProductType5c790c8ef59cc8b2 = s_41_1[(s_41_3) as usize];
        // D s_41_5: write-var ga#12413 <= s_41_4
        fn_state.ga_12413 = s_41_4;
        // D s_41_6: read-var ga#12413.0:struct
        let s_41_6: u64 = fn_state.ga_12413._0;
        // C s_41_7: const #1s : i
        let s_41_7: i128 = 1;
        // D s_41_8: read-var maskshadow#281:i
        let s_41_8: i128 = fn_state.maskshadow_281;
        // D s_41_9: sub s_41_8 s_41_7
        let s_41_9: i128 = ((s_41_8) - (s_41_7));
        // D s_41_10: cast reint s_41_9 -> i64
        let s_41_10: i64 = (s_41_9 as i64);
        // D s_41_11: cast zx s_41_6 -> bv
        let s_41_11: Bits = Bits::new(s_41_6 as u128, 64u16);
        // D s_41_12: cast zx s_41_10 -> i
        let s_41_12: i128 = (i128::try_from(s_41_10).unwrap());
        // D s_41_13: read-var bottomshadow#280:i64
        let s_41_13: i64 = fn_state.bottomshadow_280;
        // D s_41_14: cast zx s_41_13 -> i
        let s_41_14: i128 = (i128::try_from(s_41_13).unwrap());
        // D s_41_15: call is_zero_subrange(s_41_11, s_41_12, s_41_14)
        let s_41_15: bool = is_zero_subrange(state, tracer, s_41_11, s_41_12, s_41_14);
        // D s_41_16: not s_41_15
        let s_41_16: bool = !s_41_15;
        // D s_41_17: write-var gs#16658 <= s_41_16
        fn_state.gs_16658 = s_41_16;
        // N s_41_18: jump b37
        return block_37(state, tracer, fn_state);
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_42_0: read-var maskshadow#281:i
        let s_42_0: i128 = fn_state.maskshadow_281;
        // D s_42_1: call __id(s_42_0)
        let s_42_1: i128 = u__id(state, tracer, s_42_0);
        // D s_42_2: read-var topshadow#282:i
        let s_42_2: i128 = fn_state.topshadow_282;
        // D s_42_3: call __id(s_42_2)
        let s_42_3: i128 = u__id(state, tracer, s_42_2);
        // D s_42_4: cmp-le s_42_1 s_42_3
        let s_42_4: bool = ((s_42_1) <= (s_42_3));
        // N s_42_5: branch s_42_4 b45 b43
        if s_42_4 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_43(state, tracer, fn_state);
        };
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_43_0: const #0u : u8
        let s_43_0: bool = false;
        // D s_43_1: write-var gs#16655 <= s_43_0
        fn_state.gs_16655 = s_43_0;
        // N s_43_2: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_44_0: read-var gs#16655:u8
        let s_44_0: bool = fn_state.gs_16655;
        // D s_44_1: write-var gs#16656 <= s_44_0
        fn_state.gs_16656 = s_44_0;
        // N s_44_2: jump b35
        return block_35(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_45_0: read-var topshadow#282:i
        let s_45_0: i128 = fn_state.topshadow_282;
        // D s_45_1: call __id(s_45_0)
        let s_45_1: i128 = u__id(state, tracer, s_45_0);
        // C s_45_2: const #64s : i
        let s_45_2: i128 = 64;
        // D s_45_3: cmp-lt s_45_1 s_45_2
        let s_45_3: bool = ((s_45_1) < (s_45_2));
        // D s_45_4: write-var gs#16655 <= s_45_3
        fn_state.gs_16655 = s_45_3;
        // N s_45_5: jump b44
        return block_44(state, tracer, fn_state);
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_46_0: const #63s : i
        let s_46_0: i128 = 63;
        // D s_46_1: write-var top <= s_46_0
        fn_state.top = s_46_0;
        // N s_46_2: jump b20
        return block_20(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_47_0: const #71u : u32
        let s_47_0: u32 = 71;
        // S s_47_1: call ConstrainUnpredictableBool(s_47_0)
        let s_47_1: bool = ConstrainUnpredictableBool(state, tracer, s_47_0);
        // D s_47_2: write-var gs#16643 <= s_47_1
        fn_state.gs_16643 = s_47_1;
        // N s_47_3: jump b18
        return block_18(state, tracer, fn_state);
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_48_0: const #19392u : u32
        let s_48_0: u32 = 19392;
        // D s_48_1: read-reg s_48_0:[struct; 64]
        let s_48_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_48_0 as isize);
            tracer.read_register(s_48_0 as isize, value);
            value
        };
        // D s_48_2: read-var n:i64
        let s_48_2: i64 = fn_state.n;
        // D s_48_3: cast zx s_48_2 -> i
        let s_48_3: i128 = (i128::try_from(s_48_2).unwrap());
        // D s_48_4: read-element s_48_1[s_48_3]
        let s_48_4: ProductType5c790c8ef59cc8b2 = s_48_1[(s_48_3) as usize];
        // D s_48_5: write-var ga#12397 <= s_48_4
        fn_state.ga_12397 = s_48_4;
        // D s_48_6: read-var ga#12397.0:struct
        let s_48_6: u64 = fn_state.ga_12397._0;
        // C s_48_7: const #63s : i
        let s_48_7: i128 = 63;
        // D s_48_8: cast zx s_48_6 -> bv
        let s_48_8: Bits = Bits::new(s_48_6 as u128, 64u16);
        // D s_48_9: read-var top:i
        let s_48_9: i128 = fn_state.top;
        // D s_48_10: call is_zero_subrange(s_48_8, s_48_7, s_48_9)
        let s_48_10: bool = is_zero_subrange(state, tracer, s_48_8, s_48_7, s_48_9);
        // D s_48_11: not s_48_10
        let s_48_11: bool = !s_48_10;
        // D s_48_12: write-var gs#16642 <= s_48_11
        fn_state.gs_16642 = s_48_11;
        // N s_48_13: jump b16
        return block_16(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_49_0: const #19392u : u32
        let s_49_0: u32 = 19392;
        // D s_49_1: read-reg s_49_0:[struct; 64]
        let s_49_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_49_0 as isize);
            tracer.read_register(s_49_0 as isize, value);
            value
        };
        // D s_49_2: read-var n:i64
        let s_49_2: i64 = fn_state.n;
        // D s_49_3: cast zx s_49_2 -> i
        let s_49_3: i128 = (i128::try_from(s_49_2).unwrap());
        // D s_49_4: read-element s_49_1[s_49_3]
        let s_49_4: ProductType5c790c8ef59cc8b2 = s_49_1[(s_49_3) as usize];
        // D s_49_5: write-var ga#12393 <= s_49_4
        fn_state.ga_12393 = s_49_4;
        // D s_49_6: read-var ga#12393.0:struct
        let s_49_6: u64 = fn_state.ga_12393._0;
        // C s_49_7: const #63s : i
        let s_49_7: i128 = 63;
        // D s_49_8: cast zx s_49_6 -> bv
        let s_49_8: Bits = Bits::new(s_49_6 as u128, 64u16);
        // D s_49_9: read-var top:i
        let s_49_9: i128 = fn_state.top;
        // D s_49_10: call is_ones_subrange(s_49_8, s_49_7, s_49_9)
        let s_49_10: bool = is_ones_subrange(state, tracer, s_49_8, s_49_7, s_49_9);
        // D s_49_11: not s_49_10
        let s_49_11: bool = !s_49_10;
        // D s_49_12: write-var gs#16640 <= s_49_11
        fn_state.gs_16640 = s_49_11;
        // N s_49_13: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_50_0: const #3s : i
        let s_50_0: i128 = 3;
        // C s_50_1: const #31s : i
        let s_50_1: i128 = 31;
        // C s_50_2: const #29u : u32
        let s_50_2: u32 = 29;
        // S s_50_3: call ConstrainUnpredictableInteger(s_50_0, s_50_1, s_50_2)
        let s_50_3: ProductType396b95aa74979079 = ConstrainUnpredictableInteger(
            state,
            tracer,
            s_50_0,
            s_50_1,
            s_50_2,
        );
        // D s_50_4: write-var ga#12389 <= s_50_3
        fn_state.ga_12389 = s_50_3;
        // D s_50_5: read-var ga#12389.0:struct
        let s_50_5: u32 = fn_state.ga_12389._0;
        // D s_50_6: read-var ga#12389.1:struct
        let s_50_6: i128 = fn_state.ga_12389._1;
        // D s_50_7: write-var c <= s_50_5
        fn_state.c = s_50_5;
        // D s_50_8: write-var mask <= s_50_6
        fn_state.mask = s_50_6;
        // D s_50_9: read-var c:u32
        let s_50_9: u32 = fn_state.c;
        // C s_50_10: const #7u : u32
        let s_50_10: u32 = 7;
        // D s_50_11: cmp-eq s_50_9 s_50_10
        let s_50_11: bool = ((s_50_9) == (s_50_10));
        // N s_50_12: branch s_50_11 b61 b51
        if s_50_11 {
            return block_61(state, tracer, fn_state);
        } else {
            return block_51(state, tracer, fn_state);
        };
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_51_0: read-var c:u32
        let s_51_0: u32 = fn_state.c;
        // C s_51_1: const #0u : u32
        let s_51_1: u32 = 0;
        // D s_51_2: cmp-eq s_51_0 s_51_1
        let s_51_2: bool = ((s_51_0) == (s_51_1));
        // N s_51_3: branch s_51_2 b60 b52
        if s_51_2 {
            return block_60(state, tracer, fn_state);
        } else {
            return block_52(state, tracer, fn_state);
        };
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_52_0: read-var c:u32
        let s_52_0: u32 = fn_state.c;
        // C s_52_1: const #1u : u32
        let s_52_1: u32 = 1;
        // D s_52_2: cmp-eq s_52_0 s_52_1
        let s_52_2: bool = ((s_52_0) == (s_52_1));
        // D s_52_3: write-var gs#16628 <= s_52_2
        fn_state.gs_16628 = s_52_2;
        // N s_52_4: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_53_0: read-var gs#16628:u8
        let s_53_0: bool = fn_state.gs_16628;
        // D s_53_1: write-var gs#16629 <= s_53_0
        fn_state.gs_16629 = s_53_0;
        // N s_53_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_54<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // D s_54_0: read-var gs#16629:u8
        let s_54_0: bool = fn_state.gs_16629;
        // N s_54_1: assert s_54_0
        let s_54_1: () = assert!(s_54_0);
        // C s_54_2: const #7u : u32
        let s_54_2: u32 = 7;
        // D s_54_3: read-var c:u32
        let s_54_3: u32 = fn_state.c;
        // D s_54_4: cmp-eq s_54_2 s_54_3
        let s_54_4: bool = ((s_54_2) == (s_54_3));
        // D s_54_5: not s_54_4
        let s_54_5: bool = !s_54_4;
        // N s_54_6: branch s_54_5 b56 b55
        if s_54_5 {
            return block_56(state, tracer, fn_state);
        } else {
            return block_55(state, tracer, fn_state);
        };
    }
    fn block_55<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_55_0: const #0u : u8
        let s_55_0: bool = false;
        // D s_55_1: write-var return_value <= s_55_0
        fn_state.return_value = s_55_0;
        // N s_55_2: jump b27
        return block_27(state, tracer, fn_state);
    }
    fn block_56<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_56_0: const #0u : u32
        let s_56_0: u32 = 0;
        // D s_56_1: read-var c:u32
        let s_56_1: u32 = fn_state.c;
        // D s_56_2: cmp-eq s_56_0 s_56_1
        let s_56_2: bool = ((s_56_0) == (s_56_1));
        // D s_56_3: not s_56_2
        let s_56_3: bool = !s_56_2;
        // N s_56_4: branch s_56_3 b59 b57
        if s_56_3 {
            return block_59(state, tracer, fn_state);
        } else {
            return block_57(state, tracer, fn_state);
        };
    }
    fn block_57<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_57_0: const #0s : i
        let s_57_0: i128 = 0;
        // D s_57_1: write-var mask <= s_57_0
        fn_state.mask = s_57_0;
        // N s_57_2: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_58<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_58_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_59<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // N s_59_0: jump b58
        return block_58(state, tracer, fn_state);
    }
    fn block_60<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_60_0: const #1u : u8
        let s_60_0: bool = true;
        // D s_60_1: write-var gs#16628 <= s_60_0
        fn_state.gs_16628 = s_60_0;
        // N s_60_2: jump b53
        return block_53(state, tracer, fn_state);
    }
    fn block_61<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_61_0: const #1u : u8
        let s_61_0: bool = true;
        // D s_61_1: write-var gs#16629 <= s_61_0
        fn_state.gs_16629 = s_61_0;
        // N s_61_2: jump b54
        return block_54(state, tracer, fn_state);
    }
    fn block_62<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_62_0: const #2s : i
        let s_62_0: i128 = 2;
        // D s_62_1: read-var mask:i
        let s_62_1: i128 = fn_state.mask;
        // D s_62_2: cmp-le s_62_1 s_62_0
        let s_62_2: bool = ((s_62_1) <= (s_62_0));
        // D s_62_3: write-var gs#16621 <= s_62_2
        fn_state.gs_16621 = s_62_2;
        // N s_62_4: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_63<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_63_0: const #28u : u32
        let s_63_0: u32 = 28;
        // S s_63_1: call ConstrainUnpredictableBool(s_63_0)
        let s_63_1: bool = ConstrainUnpredictableBool(state, tracer, s_63_0);
        // D s_63_2: write-var byte_select_match <= s_63_1
        fn_state.byte_select_match = s_63_1;
        // C s_63_3: const #3s : i64
        let s_63_3: i64 = 3;
        // D s_63_4: write-var bottom <= s_63_3
        fn_state.bottom = s_63_3;
        // N s_63_5: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_64<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_64_0: const #27u : u32
        let s_64_0: u32 = 27;
        // S s_64_1: call ConstrainUnpredictableBool(s_64_0)
        let s_64_1: bool = ConstrainUnpredictableBool(state, tracer, s_64_0);
        // D s_64_2: write-var byte_select_match <= s_64_1
        fn_state.byte_select_match = s_64_1;
        // N s_64_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_65<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_65_0: const #103984u : u32
        let s_65_0: u32 = 103984;
        // D s_65_1: read-reg s_65_0:[struct; 64]
        let s_65_1: [ProductType5c790c8ef59cc8b2; 64usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 64usize],
                >(s_65_0 as isize);
            tracer.read_register(s_65_0 as isize, value);
            value
        };
        // D s_65_2: read-var n:i64
        let s_65_2: i64 = fn_state.n;
        // D s_65_3: cast zx s_65_2 -> i
        let s_65_3: i128 = (i128::try_from(s_65_2).unwrap());
        // D s_65_4: read-element s_65_1[s_65_3]
        let s_65_4: ProductType5c790c8ef59cc8b2 = s_65_1[(s_65_3) as usize];
        // D s_65_5: call _get_DBGWCR_EL1_Type_BAS(s_65_4)
        let s_65_5: u8 = u_get_DBGWCR_EL1_Type_BAS(state, tracer, s_65_4);
        // D s_65_6: cast zx s_65_5 -> bv
        let s_65_6: Bits = Bits::new(s_65_5 as u128, 8u16);
        // D s_65_7: call IsOnes(s_65_6)
        let s_65_7: bool = IsOnes(state, tracer, s_65_6);
        // D s_65_8: not s_65_7
        let s_65_8: bool = !s_65_7;
        // D s_65_9: write-var gs#16611 <= s_65_8
        fn_state.gs_16611 = s_65_8;
        // N s_65_10: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_66<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> bool {
        // C s_66_0: const #2s : i64
        let s_66_0: i64 = 2;
        // D s_66_1: write-var bottom <= s_66_0
        fn_state.bottom = s_66_0;
        // N s_66_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
