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
use NMRR_read::*;
use HaveAArch32EL::*;
use DecodeSDFAttr::*;
use DecodeShareability::*;
use PRRR_NS_read::*;
use u_get_PRRR_Type_NS1::*;
use ConstrainUnpredictableBits::*;
use u_get_PRRR_Type_NS0::*;
use u__IMPDEF_MemoryAttributes::*;
use PRRR_read::*;
use NMRR_NS_read::*;
use Unreachable::*;
use common::*;
pub fn AArch32_RemappedTEXDecode<T: Tracer>(
    state: &mut State,
    tracer: &T,
    regime: u32,
    TEX: u8,
    C: bool,
    B: bool,
    s: bool,
) -> ProductTypef170cab34335b70c {
    #[derive(Default)]
    struct FunctionState {
        base: i64,
        gs_28671: bool,
        ga_22216: ProductType183e6678e5239c85,
        attrfield: Bits,
        memattrs: ProductTypef170cab34335b70c,
        prrr: ProductType700c18a878c5601b,
        return_value: ProductTypef170cab34335b70c,
        ga_22202: ProductType183e6678e5239c85,
        NOSmshadow_531: bool,
        ga_22214: ProductType183e6678e5239c85,
        NSnshadow_530: bool,
        gs_28675: bool,
        ga_22200: ProductType183e6678e5239c85,
        gs_453976: ProductType690b94b58c91cec7,
        region: i64,
        nmrr: ProductType700c18a878c5601b,
        regime: u32,
        TEX: u8,
        C: bool,
        B: bool,
        s: bool,
    }
    let fn_state = FunctionState {
        regime,
        TEX,
        C,
        B,
        s,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_0_0: const #0s : i
        let s_0_0: i128 = 0;
        // D s_0_1: read-var TEX:u8
        let s_0_1: u8 = fn_state.TEX;
        // D s_0_2: cast zx s_0_1 -> bv
        let s_0_2: Bits = Bits::new(s_0_1 as u128, 3u16);
        // C s_0_3: const #1u : u64
        let s_0_3: u64 = 1;
        // D s_0_4: bit-extract s_0_2 s_0_0 s_0_3
        let s_0_4: Bits = (Bits::new(
            ((s_0_2) >> (s_0_0)).value(),
            u16::try_from(s_0_3).unwrap(),
        ));
        // D s_0_5: cast reint s_0_4 -> u8
        let s_0_5: bool = ((s_0_4.value()) != 0);
        // C s_0_6: const #0s : i
        let s_0_6: i128 = 0;
        // C s_0_7: const #0u : u64
        let s_0_7: u64 = 0;
        // D s_0_8: cast zx s_0_5 -> u64
        let s_0_8: u64 = (s_0_5 as u64);
        // C s_0_9: const #1u : u64
        let s_0_9: u64 = 1;
        // D s_0_10: and s_0_8 s_0_9
        let s_0_10: u64 = ((s_0_8) & (s_0_9));
        // D s_0_11: cmp-eq s_0_10 s_0_9
        let s_0_11: bool = ((s_0_10) == (s_0_9));
        // D s_0_12: lsl s_0_8 s_0_6
        let s_0_12: u64 = s_0_8 << s_0_6;
        // D s_0_13: or s_0_7 s_0_12
        let s_0_13: u64 = ((s_0_7) | (s_0_12));
        // D s_0_14: cmpl s_0_12
        let s_0_14: u64 = !s_0_12;
        // D s_0_15: and s_0_7 s_0_14
        let s_0_15: u64 = ((s_0_7) & (s_0_14));
        // D s_0_16: select s_0_11 s_0_13 s_0_15
        let s_0_16: u64 = if s_0_11 { s_0_13 } else { s_0_15 };
        // D s_0_17: cast trunc s_0_16 -> u8
        let s_0_17: bool = ((s_0_16) != 0);
        // D s_0_18: cast zx s_0_17 -> bv
        let s_0_18: Bits = Bits::new(s_0_17 as u128, 1u16);
        // D s_0_19: read-var C:u8
        let s_0_19: bool = fn_state.C;
        // D s_0_20: cast zx s_0_19 -> bv
        let s_0_20: Bits = Bits::new(s_0_19 as u128, 1u16);
        // D s_0_21: cast reint s_0_18 -> u128
        let s_0_21: u128 = (s_0_18.value() as u128);
        // D s_0_22: size-of s_0_18
        let s_0_22: u16 = s_0_18.length();
        // D s_0_23: cast reint s_0_20 -> u128
        let s_0_23: u128 = (s_0_20.value() as u128);
        // D s_0_24: size-of s_0_20
        let s_0_24: u16 = s_0_20.length();
        // D s_0_25: lsl s_0_21 s_0_24
        let s_0_25: u128 = s_0_21 << s_0_24;
        // D s_0_26: or s_0_25 s_0_23
        let s_0_26: u128 = ((s_0_25) | (s_0_23));
        // D s_0_27: add s_0_22 s_0_24
        let s_0_27: u16 = (s_0_22 + s_0_24);
        // D s_0_28: create-bits s_0_26 s_0_27
        let s_0_28: Bits = Bits::new(s_0_26, s_0_27);
        // D s_0_29: cast reint s_0_28 -> u8
        let s_0_29: u8 = (s_0_28.value() as u8);
        // D s_0_30: cast zx s_0_29 -> bv
        let s_0_30: Bits = Bits::new(s_0_29 as u128, 2u16);
        // D s_0_31: read-var B:u8
        let s_0_31: bool = fn_state.B;
        // D s_0_32: cast zx s_0_31 -> bv
        let s_0_32: Bits = Bits::new(s_0_31 as u128, 1u16);
        // D s_0_33: cast reint s_0_30 -> u128
        let s_0_33: u128 = (s_0_30.value() as u128);
        // D s_0_34: size-of s_0_30
        let s_0_34: u16 = s_0_30.length();
        // D s_0_35: cast reint s_0_32 -> u128
        let s_0_35: u128 = (s_0_32.value() as u128);
        // D s_0_36: size-of s_0_32
        let s_0_36: u16 = s_0_32.length();
        // D s_0_37: lsl s_0_33 s_0_36
        let s_0_37: u128 = s_0_33 << s_0_36;
        // D s_0_38: or s_0_37 s_0_35
        let s_0_38: u128 = ((s_0_37) | (s_0_35));
        // D s_0_39: add s_0_34 s_0_36
        let s_0_39: u16 = (s_0_34 + s_0_36);
        // D s_0_40: create-bits s_0_38 s_0_39
        let s_0_40: Bits = Bits::new(s_0_38, s_0_39);
        // D s_0_41: cast reint s_0_40 -> u8
        let s_0_41: u8 = (s_0_40.value() as u8);
        // D s_0_42: cast zx s_0_41 -> bv
        let s_0_42: Bits = Bits::new(s_0_41 as u128, 3u16);
        // D s_0_43: cast zx s_0_42 -> i
        let s_0_43: i128 = (s_0_42.value() as i128);
        // D s_0_44: cast reint s_0_43 -> i64
        let s_0_44: i64 = (s_0_43 as i64);
        // D s_0_45: write-var region <= s_0_44
        fn_state.region = s_0_44;
        // C s_0_46: const #6s : i
        let s_0_46: i128 = 6;
        // D s_0_47: read-var region:i64
        let s_0_47: i64 = fn_state.region;
        // D s_0_48: cast zx s_0_47 -> i
        let s_0_48: i128 = (i128::try_from(s_0_47).unwrap());
        // D s_0_49: cmp-eq s_0_48 s_0_46
        let s_0_49: bool = ((s_0_48) == (s_0_46));
        // N s_0_50: branch s_0_49 b34 b1
        if s_0_49 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_1_0: read-var regime:u32
        let s_1_0: u32 = fn_state.regime;
        // C s_1_1: const #1u : u32
        let s_1_1: u32 = 1;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b33 b2
        if s_1_2 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_2_0: const #424u : u32
        let s_2_0: u32 = 424;
        // D s_2_1: read-reg s_2_0:u8
        let s_2_1: u8 = {
            let value = state.read_register::<u8>(s_2_0 as isize);
            tracer.read_register(s_2_0 as isize, value);
            value
        };
        // D s_2_2: call HaveAArch32EL(s_2_1)
        let s_2_2: bool = HaveAArch32EL(state, tracer, s_2_1);
        // N s_2_3: branch s_2_2 b32 b3
        if s_2_2 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_3_0: const #() : ()
        let s_3_0: () = ();
        // S s_3_1: call PRRR_read(s_3_0)
        let s_3_1: ProductType700c18a878c5601b = PRRR_read(state, tracer, s_3_0);
        // D s_3_2: write-var prrr <= s_3_1
        fn_state.prrr = s_3_1;
        // C s_3_3: const #() : ()
        let s_3_3: () = ();
        // S s_3_4: call NMRR_read(s_3_3)
        let s_3_4: ProductType700c18a878c5601b = NMRR_read(state, tracer, s_3_3);
        // D s_3_5: write-var nmrr <= s_3_4
        fn_state.nmrr = s_3_4;
        // N s_3_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_4_0: const #2s : i
        let s_4_0: i128 = 2;
        // D s_4_1: read-var region:i64
        let s_4_1: i64 = fn_state.region;
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // D s_4_3: mul s_4_0 s_4_2
        let s_4_3: i128 = ((s_4_0) * (s_4_2));
        // D s_4_4: cast reint s_4_3 -> i64
        let s_4_4: i64 = (s_4_3 as i64);
        // D s_4_5: write-var base <= s_4_4
        fn_state.base = s_4_4;
        // D s_4_6: read-var prrr.0:struct
        let s_4_6: u32 = fn_state.prrr._0;
        // C s_4_7: const #2s : i
        let s_4_7: i128 = 2;
        // D s_4_8: cast zx s_4_6 -> bv
        let s_4_8: Bits = Bits::new(s_4_6 as u128, 32u16);
        // D s_4_9: read-var base:i64
        let s_4_9: i64 = fn_state.base;
        // D s_4_10: cast zx s_4_9 -> i
        let s_4_10: i128 = (i128::try_from(s_4_9).unwrap());
        // D s_4_11: bit-extract s_4_8 s_4_10 s_4_7
        let s_4_11: Bits = (Bits::new(
            ((s_4_8) >> (s_4_10)).value(),
            u16::try_from(s_4_7).unwrap(),
        ));
        // D s_4_12: write-var attrfield <= s_4_11
        fn_state.attrfield = s_4_11;
        // C s_4_13: const #3u : u8
        let s_4_13: u8 = 3;
        // C s_4_14: cast zx s_4_13 -> bv
        let s_4_14: Bits = Bits::new(s_4_13 as u128, 2u16);
        // D s_4_15: read-var attrfield:bv
        let s_4_15: Bits = fn_state.attrfield;
        // D s_4_16: cmp-eq s_4_15 s_4_14
        let s_4_16: bool = ((s_4_15) == (s_4_14));
        // N s_4_17: branch s_4_16 b31 b5
        if s_4_16 {
            return block_31(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // N s_5_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_6_0: read-var attrfield:bv
        let s_6_0: Bits = fn_state.attrfield;
        // C s_6_1: const #0u : u8
        let s_6_1: u8 = 0;
        // C s_6_2: cast zx s_6_1 -> bv
        let s_6_2: Bits = Bits::new(s_6_1 as u128, 2u16);
        // D s_6_3: cmp-eq s_6_0 s_6_2
        let s_6_3: bool = ((s_6_0) == (s_6_2));
        // D s_6_4: not s_6_3
        let s_6_4: bool = !s_6_3;
        // N s_6_5: branch s_6_4 b16 b7
        if s_6_4 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_7_0: const #1u : u32
        let s_7_0: u32 = 1;
        // D s_7_1: write-var memattrs.2 <= s_7_0
        fn_state.memattrs._2 = s_7_0;
        // C s_7_2: const #3u : u32
        let s_7_2: u32 = 3;
        // D s_7_3: write-var memattrs.0 <= s_7_2
        fn_state.memattrs._0 = s_7_2;
        // C s_7_4: const #2u : u32
        let s_7_4: u32 = 2;
        // D s_7_5: write-var memattrs.5 <= s_7_4
        fn_state.memattrs._5 = s_7_4;
        // N s_7_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var memattrs.1.2 <= s_8_0
        fn_state.memattrs._1._2 = s_8_0;
        // C s_8_2: const #0u : u8
        let s_8_2: bool = false;
        // D s_8_3: write-var memattrs.4.2 <= s_8_2
        fn_state.memattrs._4._2 = s_8_2;
        // C s_8_4: const #0u : u32
        let s_8_4: u32 = 0;
        // D s_8_5: write-var memattrs.6 <= s_8_4
        fn_state.memattrs._6 = s_8_4;
        // D s_8_6: read-var memattrs.1:struct
        let s_8_6: ProductType183e6678e5239c85 = fn_state.memattrs._1;
        // D s_8_7: write-var ga#22214 <= s_8_6
        fn_state.ga_22214 = s_8_6;
        // D s_8_8: read-var ga#22214.0:struct
        let s_8_8: u8 = fn_state.ga_22214._0;
        // D s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 2u16);
        // C s_8_10: const #480u : u32
        let s_8_10: u32 = 480;
        // D s_8_11: read-reg s_8_10:u8
        let s_8_11: u8 = {
            let value = state.read_register::<u8>(s_8_10 as isize);
            tracer.read_register(s_8_10 as isize, value);
            value
        };
        // D s_8_12: cast zx s_8_11 -> bv
        let s_8_12: Bits = Bits::new(s_8_11 as u128, 2u16);
        // D s_8_13: cmp-eq s_8_9 s_8_12
        let s_8_13: bool = ((s_8_9) == (s_8_12));
        // N s_8_14: branch s_8_13 b15 b9
        if s_8_13 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_9_0: const #0u : u8
        let s_9_0: bool = false;
        // D s_9_1: write-var gs#28675 <= s_9_0
        fn_state.gs_28675 = s_9_0;
        // N s_9_2: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_10_0: read-var gs#28675:u8
        let s_10_0: bool = fn_state.gs_28675;
        // N s_10_1: branch s_10_0 b14 b11
        if s_10_0 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_11_0: const #1u : u8
        let s_11_0: bool = true;
        // D s_11_1: write-var memattrs.7 <= s_11_0
        fn_state.memattrs._7 = s_11_0;
        // N s_11_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_12_0: read-var memattrs:struct
        let s_12_0: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_12_1: write-var return_value <= s_12_0
        fn_state.return_value = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_13_0: read-var return_value:struct
        let s_13_0: ProductTypef170cab34335b70c = fn_state.return_value;
        // N s_13_1: return s_13_0
        return s_13_0;
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_14_0: const #0u : u8
        let s_14_0: bool = false;
        // D s_14_1: write-var memattrs.7 <= s_14_0
        fn_state.memattrs._7 = s_14_0;
        // N s_14_2: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_15_0: read-var memattrs.4:struct
        let s_15_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_15_1: write-var ga#22216 <= s_15_0
        fn_state.ga_22216 = s_15_0;
        // D s_15_2: read-var ga#22216.0:struct
        let s_15_2: u8 = fn_state.ga_22216._0;
        // D s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 2u16);
        // C s_15_4: const #480u : u32
        let s_15_4: u32 = 480;
        // D s_15_5: read-reg s_15_4:u8
        let s_15_5: u8 = {
            let value = state.read_register::<u8>(s_15_4 as isize);
            tracer.read_register(s_15_4 as isize, value);
            value
        };
        // D s_15_6: cast zx s_15_5 -> bv
        let s_15_6: Bits = Bits::new(s_15_5 as u128, 2u16);
        // D s_15_7: cmp-eq s_15_3 s_15_6
        let s_15_7: bool = ((s_15_3) == (s_15_6));
        // D s_15_8: write-var gs#28675 <= s_15_7
        fn_state.gs_28675 = s_15_7;
        // N s_15_9: jump b10
        return block_10(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_16_0: read-var attrfield:bv
        let s_16_0: Bits = fn_state.attrfield;
        // C s_16_1: const #1u : u8
        let s_16_1: u8 = 1;
        // C s_16_2: cast zx s_16_1 -> bv
        let s_16_2: Bits = Bits::new(s_16_1 as u128, 2u16);
        // D s_16_3: cmp-eq s_16_0 s_16_2
        let s_16_3: bool = ((s_16_0) == (s_16_2));
        // D s_16_4: not s_16_3
        let s_16_4: bool = !s_16_3;
        // N s_16_5: branch s_16_4 b18 b17
        if s_16_4 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_17_0: const #1u : u32
        let s_17_0: u32 = 1;
        // D s_17_1: write-var memattrs.2 <= s_17_0
        fn_state.memattrs._2 = s_17_0;
        // C s_17_2: const #2u : u32
        let s_17_2: u32 = 2;
        // D s_17_3: write-var memattrs.0 <= s_17_2
        fn_state.memattrs._0 = s_17_2;
        // C s_17_4: const #2u : u32
        let s_17_4: u32 = 2;
        // D s_17_5: write-var memattrs.5 <= s_17_4
        fn_state.memattrs._5 = s_17_4;
        // N s_17_6: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_18_0: read-var attrfield:bv
        let s_18_0: Bits = fn_state.attrfield;
        // C s_18_1: const #2u : u8
        let s_18_1: u8 = 2;
        // C s_18_2: cast zx s_18_1 -> bv
        let s_18_2: Bits = Bits::new(s_18_1 as u128, 2u16);
        // D s_18_3: cmp-eq s_18_0 s_18_2
        let s_18_3: bool = ((s_18_0) == (s_18_2));
        // D s_18_4: not s_18_3
        let s_18_4: bool = !s_18_3;
        // N s_18_5: branch s_18_4 b28 b19
        if s_18_4 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_19_0: read-var s:u8
        let s_19_0: bool = fn_state.s;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 1u16);
        // C s_19_2: const #0u : u8
        let s_19_2: bool = false;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 1u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // N s_19_5: branch s_19_4 b27 b20
        if s_19_4 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_20_0: read-var prrr:struct
        let s_20_0: ProductType700c18a878c5601b = fn_state.prrr;
        // D s_20_1: call _get_PRRR_Type_NS1(s_20_0)
        let s_20_1: bool = u_get_PRRR_Type_NS1(state, tracer, s_20_0);
        // D s_20_2: write-var NSnshadow#530 <= s_20_1
        fn_state.NSnshadow_530 = s_20_1;
        // N s_20_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_21_0: read-var prrr.0:struct
        let s_21_0: u32 = fn_state.prrr._0;
        // C s_21_1: const #24s : i
        let s_21_1: i128 = 24;
        // D s_21_2: read-var region:i64
        let s_21_2: i64 = fn_state.region;
        // D s_21_3: cast zx s_21_2 -> i
        let s_21_3: i128 = (i128::try_from(s_21_2).unwrap());
        // D s_21_4: add s_21_3 s_21_1
        let s_21_4: i128 = (s_21_3 + s_21_1);
        // D s_21_5: cast reint s_21_4 -> i64
        let s_21_5: i64 = (s_21_4 as i64);
        // D s_21_6: cast zx s_21_0 -> bv
        let s_21_6: Bits = Bits::new(s_21_0 as u128, 32u16);
        // D s_21_7: cast zx s_21_5 -> i
        let s_21_7: i128 = (i128::try_from(s_21_5).unwrap());
        // C s_21_8: const #1u : u64
        let s_21_8: u64 = 1;
        // D s_21_9: bit-extract s_21_6 s_21_7 s_21_8
        let s_21_9: Bits = (Bits::new(
            ((s_21_6) >> (s_21_7)).value(),
            u16::try_from(s_21_8).unwrap(),
        ));
        // D s_21_10: cast reint s_21_9 -> u8
        let s_21_10: bool = ((s_21_9.value()) != 0);
        // C s_21_11: const #0s : i
        let s_21_11: i128 = 0;
        // C s_21_12: const #0u : u64
        let s_21_12: u64 = 0;
        // D s_21_13: cast zx s_21_10 -> u64
        let s_21_13: u64 = (s_21_10 as u64);
        // C s_21_14: const #1u : u64
        let s_21_14: u64 = 1;
        // D s_21_15: and s_21_13 s_21_14
        let s_21_15: u64 = ((s_21_13) & (s_21_14));
        // D s_21_16: cmp-eq s_21_15 s_21_14
        let s_21_16: bool = ((s_21_15) == (s_21_14));
        // D s_21_17: lsl s_21_13 s_21_11
        let s_21_17: u64 = s_21_13 << s_21_11;
        // D s_21_18: or s_21_12 s_21_17
        let s_21_18: u64 = ((s_21_12) | (s_21_17));
        // D s_21_19: cmpl s_21_17
        let s_21_19: u64 = !s_21_17;
        // D s_21_20: and s_21_12 s_21_19
        let s_21_20: u64 = ((s_21_12) & (s_21_19));
        // D s_21_21: select s_21_16 s_21_18 s_21_20
        let s_21_21: u64 = if s_21_16 { s_21_18 } else { s_21_20 };
        // D s_21_22: cast trunc s_21_21 -> u8
        let s_21_22: bool = ((s_21_21) != 0);
        // D s_21_23: cast zx s_21_22 -> bv
        let s_21_23: Bits = Bits::new(s_21_22 as u128, 1u16);
        // D s_21_24: read-var NSnshadow#530:u8
        let s_21_24: bool = fn_state.NSnshadow_530;
        // D s_21_25: cast zx s_21_24 -> bv
        let s_21_25: Bits = Bits::new(s_21_24 as u128, 1u16);
        // D s_21_26: and s_21_23 s_21_25
        let s_21_26: Bits = ((s_21_23) & (s_21_25));
        // D s_21_27: cast reint s_21_26 -> u8
        let s_21_27: bool = ((s_21_26.value()) != 0);
        // D s_21_28: write-var NOSmshadow#531 <= s_21_27
        fn_state.NOSmshadow_531 = s_21_27;
        // D s_21_29: read-var nmrr.0:struct
        let s_21_29: u32 = fn_state.nmrr._0;
        // C s_21_30: const #2s : i
        let s_21_30: i128 = 2;
        // D s_21_31: cast zx s_21_29 -> bv
        let s_21_31: Bits = Bits::new(s_21_29 as u128, 32u16);
        // D s_21_32: read-var base:i64
        let s_21_32: i64 = fn_state.base;
        // D s_21_33: cast zx s_21_32 -> i
        let s_21_33: i128 = (i128::try_from(s_21_32).unwrap());
        // D s_21_34: bit-extract s_21_31 s_21_33 s_21_30
        let s_21_34: Bits = (Bits::new(
            ((s_21_31) >> (s_21_33)).value(),
            u16::try_from(s_21_30).unwrap(),
        ));
        // D s_21_35: read-var nmrr.0:struct
        let s_21_35: u32 = fn_state.nmrr._0;
        // C s_21_36: const #16s : i
        let s_21_36: i128 = 16;
        // D s_21_37: read-var base:i64
        let s_21_37: i64 = fn_state.base;
        // D s_21_38: cast zx s_21_37 -> i
        let s_21_38: i128 = (i128::try_from(s_21_37).unwrap());
        // D s_21_39: add s_21_38 s_21_36
        let s_21_39: i128 = (s_21_38 + s_21_36);
        // D s_21_40: cast reint s_21_39 -> i64
        let s_21_40: i64 = (s_21_39 as i64);
        // C s_21_41: const #2s : i
        let s_21_41: i128 = 2;
        // D s_21_42: cast zx s_21_35 -> bv
        let s_21_42: Bits = Bits::new(s_21_35 as u128, 32u16);
        // D s_21_43: cast zx s_21_40 -> i
        let s_21_43: i128 = (i128::try_from(s_21_40).unwrap());
        // D s_21_44: bit-extract s_21_42 s_21_43 s_21_41
        let s_21_44: Bits = (Bits::new(
            ((s_21_42) >> (s_21_43)).value(),
            u16::try_from(s_21_41).unwrap(),
        ));
        // C s_21_45: const #0u : u32
        let s_21_45: u32 = 0;
        // D s_21_46: write-var memattrs.2 <= s_21_45
        fn_state.memattrs._2 = s_21_45;
        // D s_21_47: cast reint s_21_34 -> u8
        let s_21_47: u8 = (s_21_34.value() as u8);
        // D s_21_48: call DecodeSDFAttr(s_21_47)
        let s_21_48: ProductType183e6678e5239c85 = DecodeSDFAttr(state, tracer, s_21_47);
        // D s_21_49: write-var memattrs.1 <= s_21_48
        fn_state.memattrs._1 = s_21_48;
        // D s_21_50: cast reint s_21_44 -> u8
        let s_21_50: u8 = (s_21_44.value() as u8);
        // D s_21_51: call DecodeSDFAttr(s_21_50)
        let s_21_51: ProductType183e6678e5239c85 = DecodeSDFAttr(state, tracer, s_21_50);
        // D s_21_52: write-var memattrs.4 <= s_21_51
        fn_state.memattrs._4 = s_21_51;
        // D s_21_53: read-var memattrs.1:struct
        let s_21_53: ProductType183e6678e5239c85 = fn_state.memattrs._1;
        // D s_21_54: write-var ga#22200 <= s_21_53
        fn_state.ga_22200 = s_21_53;
        // D s_21_55: read-var ga#22200.0:struct
        let s_21_55: u8 = fn_state.ga_22200._0;
        // D s_21_56: cast zx s_21_55 -> bv
        let s_21_56: Bits = Bits::new(s_21_55 as u128, 2u16);
        // C s_21_57: const #464u : u32
        let s_21_57: u32 = 464;
        // D s_21_58: read-reg s_21_57:u8
        let s_21_58: u8 = {
            let value = state.read_register::<u8>(s_21_57 as isize);
            tracer.read_register(s_21_57 as isize, value);
            value
        };
        // D s_21_59: cast zx s_21_58 -> bv
        let s_21_59: Bits = Bits::new(s_21_58 as u128, 2u16);
        // D s_21_60: cmp-eq s_21_56 s_21_59
        let s_21_60: bool = ((s_21_56) == (s_21_59));
        // N s_21_61: branch s_21_60 b26 b22
        if s_21_60 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_22_0: const #0u : u8
        let s_22_0: bool = false;
        // D s_22_1: write-var gs#28671 <= s_22_0
        fn_state.gs_28671 = s_22_0;
        // N s_22_2: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_23_0: read-var gs#28671:u8
        let s_23_0: bool = fn_state.gs_28671;
        // N s_23_1: branch s_23_0 b25 b24
        if s_23_0 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_24_0: read-var NSnshadow#530:u8
        let s_24_0: bool = fn_state.NSnshadow_530;
        // D s_24_1: cast zx s_24_0 -> bv
        let s_24_1: Bits = Bits::new(s_24_0 as u128, 1u16);
        // D s_24_2: read-var NOSmshadow#531:u8
        let s_24_2: bool = fn_state.NOSmshadow_531;
        // D s_24_3: cast zx s_24_2 -> bv
        let s_24_3: Bits = Bits::new(s_24_2 as u128, 1u16);
        // D s_24_4: cast reint s_24_1 -> u128
        let s_24_4: u128 = (s_24_1.value() as u128);
        // D s_24_5: size-of s_24_1
        let s_24_5: u16 = s_24_1.length();
        // D s_24_6: cast reint s_24_3 -> u128
        let s_24_6: u128 = (s_24_3.value() as u128);
        // D s_24_7: size-of s_24_3
        let s_24_7: u16 = s_24_3.length();
        // D s_24_8: lsl s_24_4 s_24_7
        let s_24_8: u128 = s_24_4 << s_24_7;
        // D s_24_9: or s_24_8 s_24_6
        let s_24_9: u128 = ((s_24_8) | (s_24_6));
        // D s_24_10: add s_24_5 s_24_7
        let s_24_10: u16 = (s_24_5 + s_24_7);
        // D s_24_11: create-bits s_24_9 s_24_10
        let s_24_11: Bits = Bits::new(s_24_9, s_24_10);
        // D s_24_12: cast reint s_24_11 -> u8
        let s_24_12: u8 = (s_24_11.value() as u8);
        // D s_24_13: call DecodeShareability(s_24_12)
        let s_24_13: u32 = DecodeShareability(state, tracer, s_24_12);
        // D s_24_14: write-var memattrs.5 <= s_24_13
        fn_state.memattrs._5 = s_24_13;
        // N s_24_15: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_25_0: const #2u : u32
        let s_25_0: u32 = 2;
        // D s_25_1: write-var memattrs.5 <= s_25_0
        fn_state.memattrs._5 = s_25_0;
        // N s_25_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_26_0: read-var memattrs.4:struct
        let s_26_0: ProductType183e6678e5239c85 = fn_state.memattrs._4;
        // D s_26_1: write-var ga#22202 <= s_26_0
        fn_state.ga_22202 = s_26_0;
        // D s_26_2: read-var ga#22202.0:struct
        let s_26_2: u8 = fn_state.ga_22202._0;
        // D s_26_3: cast zx s_26_2 -> bv
        let s_26_3: Bits = Bits::new(s_26_2 as u128, 2u16);
        // C s_26_4: const #464u : u32
        let s_26_4: u32 = 464;
        // D s_26_5: read-reg s_26_4:u8
        let s_26_5: u8 = {
            let value = state.read_register::<u8>(s_26_4 as isize);
            tracer.read_register(s_26_4 as isize, value);
            value
        };
        // D s_26_6: cast zx s_26_5 -> bv
        let s_26_6: Bits = Bits::new(s_26_5 as u128, 2u16);
        // D s_26_7: cmp-eq s_26_3 s_26_6
        let s_26_7: bool = ((s_26_3) == (s_26_6));
        // D s_26_8: write-var gs#28671 <= s_26_7
        fn_state.gs_28671 = s_26_7;
        // N s_26_9: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_27_0: read-var prrr:struct
        let s_27_0: ProductType700c18a878c5601b = fn_state.prrr;
        // D s_27_1: call _get_PRRR_Type_NS0(s_27_0)
        let s_27_1: bool = u_get_PRRR_Type_NS0(state, tracer, s_27_0);
        // D s_27_2: write-var NSnshadow#530 <= s_27_1
        fn_state.NSnshadow_530 = s_27_1;
        // N s_27_3: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_28_0: read-var attrfield:bv
        let s_28_0: Bits = fn_state.attrfield;
        // C s_28_1: const #3u : u8
        let s_28_1: u8 = 3;
        // C s_28_2: cast zx s_28_1 -> bv
        let s_28_2: Bits = Bits::new(s_28_1 as u128, 2u16);
        // D s_28_3: cmp-eq s_28_0 s_28_2
        let s_28_3: bool = ((s_28_0) == (s_28_2));
        // D s_28_4: not s_28_3
        let s_28_4: bool = !s_28_3;
        // N s_28_5: branch s_28_4 b30 b29
        if s_28_4 {
            return block_30(state, tracer, fn_state);
        } else {
            return block_29(state, tracer, fn_state);
        };
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_29_0: const #() : ()
        let s_29_0: () = ();
        // S s_29_1: call Unreachable(s_29_0)
        let s_29_1: () = Unreachable(state, tracer, s_29_0);
        // N s_29_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // N s_30_0: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_31_0: const #2s : i
        let s_31_0: i128 = 2;
        // C s_31_1: const #13u : u32
        let s_31_1: u32 = 13;
        // S s_31_2: call ConstrainUnpredictableBits(s_31_1, s_31_0)
        let s_31_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_31_1,
            s_31_0,
        );
        // D s_31_3: write-var gs#453976 <= s_31_2
        fn_state.gs_453976 = s_31_2;
        // D s_31_4: read-var gs#453976.1:struct
        let s_31_4: Bits = fn_state.gs_453976._1;
        // D s_31_5: cast reint s_31_4 -> u8
        let s_31_5: u8 = (s_31_4.value() as u8);
        // D s_31_6: cast zx s_31_5 -> bv
        let s_31_6: Bits = Bits::new(s_31_5 as u128, 2u16);
        // D s_31_7: write-var attrfield <= s_31_6
        fn_state.attrfield = s_31_6;
        // N s_31_8: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_32_0: const #() : ()
        let s_32_0: () = ();
        // S s_32_1: call PRRR_NS_read(s_32_0)
        let s_32_1: ProductType700c18a878c5601b = PRRR_NS_read(state, tracer, s_32_0);
        // D s_32_2: write-var prrr <= s_32_1
        fn_state.prrr = s_32_1;
        // C s_32_3: const #() : ()
        let s_32_3: () = ();
        // S s_32_4: call NMRR_NS_read(s_32_3)
        let s_32_4: ProductType700c18a878c5601b = NMRR_NS_read(state, tracer, s_32_3);
        // D s_32_5: write-var nmrr <= s_32_4
        fn_state.nmrr = s_32_4;
        // N s_32_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_33_0: const #19984u : u32
        let s_33_0: u32 = 19984;
        // D s_33_1: read-reg s_33_0:struct
        let s_33_1: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_33_0 as isize);
            tracer.read_register(s_33_0 as isize, value);
            value
        };
        // D s_33_2: write-var prrr <= s_33_1
        fn_state.prrr = s_33_1;
        // C s_33_3: const #100840u : u32
        let s_33_3: u32 = 100840;
        // D s_33_4: read-reg s_33_3:struct
        let s_33_4: ProductType700c18a878c5601b = {
            let value = state
                .read_register::<ProductType700c18a878c5601b>(s_33_3 as isize);
            tracer.read_register(s_33_3 as isize, value);
            value
        };
        // D s_33_5: write-var nmrr <= s_33_4
        fn_state.nmrr = s_33_4;
        // N s_33_6: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_34_0: const #"" : str
        let s_34_0: &'static str = "";
        // S s_34_1: call __IMPDEF_MemoryAttributes(s_34_0)
        let s_34_1: ProductTypef170cab34335b70c = u__IMPDEF_MemoryAttributes(
            state,
            tracer,
            s_34_0,
        );
        // D s_34_2: write-var return_value <= s_34_1
        fn_state.return_value = s_34_1;
        // N s_34_3: jump b13
        return block_13(state, tracer, fn_state);
    }
}
